// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use std::cell::RefCell;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TrySendError};

use splinter::events::{Igniter, WebSocketClient, WebSocketError, WsResponse};
use splinter::service::scabbard::{StateChange as ScabbardStateChange, StateChangeEvent};

use crate::event::{
    CommitEvent, EventConnection, EventConnectionUnsubscriber, EventIoError, StateChange,
};

#[derive(Debug)]
pub struct ScabbardEventConnectionError(pub String);

impl std::error::Error for ScabbardEventConnectionError {}

impl std::fmt::Display for ScabbardEventConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "unable to create connection to {}", self.0)
    }
}

/// Constructs ScabbardEventConnections to receive events.
pub struct ScabbardEventConnectionFactory {
    node_endpoint: String,
    igniter: Igniter,
}

impl ScabbardEventConnectionFactory {
    /// Construct a new factory connecting to a specific splinter node.
    pub fn new(node_endpoint: &str, igniter: Igniter) -> Self {
        Self {
            node_endpoint: node_endpoint.into(),
            igniter,
        }
    }

    /// Create a ScabbardEventConnection on a given circuit and service.
    pub fn create_connection(
        &self,
        circuit_id: &str,
        service_id: &str,
    ) -> Result<ScabbardEventConnection, ScabbardEventConnectionError> {
        let source = format!("{}::{}", circuit_id, service_id);
        let connection_url = format!(
            "{}/scabbard/{}/{}/ws/subscribe",
            self.node_endpoint, circuit_id, service_id
        );

        Ok(ScabbardEventConnection::new(
            source,
            connection_url,
            self.igniter.clone(),
        ))
    }
}

enum ConnectionState {
    Connected(Receiver<ConnectionCommand>),
    Disconnected,
}

enum ConnectionCommand {
    Message(StateChangeEvent),
    Shutdown,
}

/// An event connection for Scabbard state events.
pub struct ScabbardEventConnection {
    name: String,
    connection_url: String,
    igniter: Igniter,
    connection_state: RefCell<ConnectionState>,
}

impl ScabbardEventConnection {
    fn new(name: String, connection_url: String, igniter: Igniter) -> Self {
        Self {
            name,
            connection_url,
            igniter,
            connection_state: RefCell::new(ConnectionState::Disconnected),
        }
    }
}

impl EventConnection for ScabbardEventConnection {
    type Unsubscriber = ScabbardEventUnsubscriber;

    fn name(&self) -> &str {
        &self.name
    }

    fn subscribe(
        &mut self,
        _namespaces: &[&str],
        last_commit_id: Option<&str>,
    ) -> Result<Self::Unsubscriber, EventIoError> {
        let (sender, receiver) = sync_channel(128);

        let source = self.name.clone();
        let unsubscribe_sender = sender.clone();
        let url = if let Some(last_commit_id) = last_commit_id {
            format!("{}?last_seen_event={}", self.connection_url, last_commit_id)
        } else {
            self.connection_url.clone()
        };
        let mut state_delta_ws = WebSocketClient::new(&url, move |_, event: StateChangeEvent| {
            match sender.try_send(ConnectionCommand::Message(event)) {
                Ok(_) => (),
                Err(TrySendError::Full(ConnectionCommand::Message(event))) => {
                    error!(
                        "dropping commit event {} from {} due to back pressure",
                        event.id, source
                    );
                }
                Err(TrySendError::Full(ConnectionCommand::Shutdown)) => {
                    // This shouldn't happen, since we never send this type
                    unreachable!()
                }
                Err(TrySendError::Disconnected(_)) => return WsResponse::Close,
            }
            WsResponse::Empty
        });

        state_delta_ws.on_error(move |err, ctx| {
            error!(
                "An error occurred while listening for scabbard events {}",
                err
            );
            if let WebSocketError::ParserError { .. } = err {
                debug!("Protocol error, closing connection");
                Ok(())
            } else {
                debug!("Attempting to restart connection");
                ctx.start_ws()
            }
        });
        self.igniter.start_ws(&state_delta_ws).map_err(|err| {
            EventIoError::ConnectionError(format!("unable to connect to web socket: {}", err))
        })?;

        let mut connection_state = self.connection_state.borrow_mut();
        *connection_state = ConnectionState::Connected(receiver);

        Ok(ScabbardEventUnsubscriber {
            name: self.name.clone(),
            unsubscribe_sender,
        })
    }

    fn recv(&self) -> Result<CommitEvent, EventIoError> {
        let mut connection_state = self.connection_state.borrow_mut();
        match *connection_state {
            ConnectionState::Connected(ref receiver) => match receiver.recv() {
                Ok(ConnectionCommand::Message(scabbard_evt)) => Ok(CommitEvent {
                    service_id: Some(self.name.clone()),
                    id: scabbard_evt.id,
                    height: None,
                    state_changes: scabbard_evt
                        .state_changes
                        .into_iter()
                        .map(|state_change| match state_change {
                            ScabbardStateChange::Set { key, value } => {
                                StateChange::Set { key, value }
                            }
                            ScabbardStateChange::Delete { key } => StateChange::Delete { key },
                        })
                        .collect(),
                }),
                Ok(ConnectionCommand::Shutdown) => {
                    debug!("Disconnecting event connection to {}", self.name);

                    *connection_state = ConnectionState::Disconnected;

                    Err(EventIoError::ConnectionError(format!(
                        "event connection to {} has closed",
                        self.name
                    )))
                }
                Err(_) => Err(EventIoError::ConnectionError(format!(
                    "event connection to {} has closed",
                    self.name
                ))),
            },
            ConnectionState::Disconnected => Err(EventIoError::ConnectionError(format!(
                "event connection to {} has closed",
                self.name
            ))),
        }
    }

    fn close(self) -> Result<(), EventIoError> {
        Ok(())
    }
}

/// EventConnectionUnsubscriber for Scabbard.
pub struct ScabbardEventUnsubscriber {
    name: String,
    unsubscribe_sender: SyncSender<ConnectionCommand>,
}

impl EventConnectionUnsubscriber for ScabbardEventUnsubscriber {
    fn unsubscribe(self) -> Result<(), EventIoError> {
        if self
            .unsubscribe_sender
            .send(ConnectionCommand::Shutdown)
            .is_err()
        {
            warn!(
                "Unable to unsubscribe from {}: already disconnected",
                self.name
            );
        }
        Ok(())
    }
}
