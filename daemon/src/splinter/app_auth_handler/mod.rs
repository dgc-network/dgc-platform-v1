// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

pub mod error;
mod node;
mod sabre;

use std::collections::HashMap;

use splinter::{
    admin::messages::AdminServiceEvent,
    events::{Igniter, ParseBytes, ParseError, WebSocketClient, WebSocketError, WsResponse},
};

use crate::database::ConnectionPool;
use crate::event::{db_handler::DatabaseEventHandler, EventProcessor};
use crate::splinter::{
    app_auth_handler::{error::AppAuthHandlerError, node::get_node_id, sabre::setup_grid},
    event::ScabbardEventConnectionFactory,
};

/// default value if the client should attempt to reconnet if ws connection is lost
const RECONNECT: bool = true;

/// default limit for number of consecutives failed reconnection attempts
const RECONNECT_LIMIT: u64 = 10;

/// default timeout in seconds if no message is received from server
const CONNECTION_TIMEOUT: u64 = 60;

#[derive(Deserialize, Debug, Clone)]
struct AdminEvent {
    timestamp: u64,

    #[serde(flatten)]
    admin_event: AdminServiceEvent,
}

impl ParseBytes<AdminEvent> for AdminEvent {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ParseError> {
        serde_json::from_slice(bytes).map_err(|err| ParseError::MalformedMessage(Box::new(err)))
    }
}

pub fn run(
    splinterd_url: String,
    event_connection_factory: ScabbardEventConnectionFactory,
    connection_pool: ConnectionPool,
    igniter: Igniter,
    scabbard_admin_key: String,
) -> Result<(), AppAuthHandlerError> {
    let registration_route = format!("{}/ws/admin/register/dgc-platform", &splinterd_url);

    let node_id = get_node_id(splinterd_url.clone())?;

    let mut ws = WebSocketClient::new(&registration_route, move |_ctx, event| {
        if let Err(err) = process_admin_event(
            event,
            &event_connection_factory,
            &connection_pool,
            &node_id,
            &scabbard_admin_key,
            &splinterd_url,
        ) {
            error!("Failed to process admin event: {}", err);
        }
        WsResponse::Empty
    });

    ws.set_reconnect(RECONNECT);
    ws.set_reconnect_limit(RECONNECT_LIMIT);
    ws.set_timeout(CONNECTION_TIMEOUT);

    ws.on_error(move |err, ctx| {
        error!("An error occured while listening for admin events {}", err);
        match err {
            WebSocketError::ParserError { .. } => {
                debug!("Protocol error, closing connection");
                Ok(())
            }
            WebSocketError::ReconnectError(_) => {
                debug!("Failed to reconnect. Closing WebSocket.");
                Ok(())
            }
            _ => {
                debug!("Attempting to restart connection");
                ctx.start_ws()
            }
        }
    });
    igniter.start_ws(&ws).map_err(AppAuthHandlerError::from)
}

fn process_admin_event(
    event: AdminEvent,
    event_connection_factory: &ScabbardEventConnectionFactory,
    connection_pool: &ConnectionPool,
    node_id: &str,
    scabbard_admin_key: &str,
    splinterd_url: &str,
) -> Result<(), AppAuthHandlerError> {
    debug!("Received the event at {}", event.timestamp);
    match event.admin_event {
        AdminServiceEvent::CircuitReady(msg_proposal) => {
            let service = match msg_proposal.circuit.roster.iter().find_map(|service| {
                if service.allowed_nodes.contains(&node_id.to_string()) {
                    Some(service)
                } else {
                    None
                }
            }) {
                Some(service) => service,
                None => {
                    debug!(
                        "New circuit does not have any services for this node: {}",
                        node_id
                    );
                    return Ok(());
                }
            };

            let scabbard_args: HashMap<_, _> = service.arguments.iter().cloned().collect();

            let proposed_admin_pubkeys = scabbard_args
                .get("admin_keys")
                .ok_or_else(|| {
                    AppAuthHandlerError::InvalidMessageError(
                        "Scabbard Service is not properly configured with \"admin_keys\" argument."
                            .into(),
                    )
                })
                .and_then(|keys_str| {
                    serde_json::from_str::<Vec<String>>(keys_str).map_err(|err| {
                        AppAuthHandlerError::InvalidMessageError(format!(
                            "unable to parse application metadata: {}",
                            err
                        ))
                    })
                })?;

            let event_connection = event_connection_factory
                .create_connection(&msg_proposal.circuit_id, &service.service_id)?;

            EventProcessor::start(
                event_connection,
                None,
                vec![Box::new(DatabaseEventHandler::new(connection_pool.clone()))],
            )
            .map_err(|err| AppAuthHandlerError::EventProcessorError(err.0))?;

            setup_grid(
                scabbard_admin_key,
                proposed_admin_pubkeys,
                &splinterd_url,
                &service.service_id,
                &msg_proposal.circuit_id,
            )?;
            Ok(())
        }
        _ => Ok(()),
    }
}
