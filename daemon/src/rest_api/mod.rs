// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

pub mod error;
mod routes;

use std::sync::mpsc;
use std::thread;

use crate::config::Endpoint;
use crate::database::ConnectionPool;
pub use crate::rest_api::error::RestApiServerError;
use crate::rest_api::routes::DbExecutor;
use crate::rest_api::routes::{
    fetch_agent, fetch_grid_schema, fetch_organization, fetch_product, fetch_record,
    fetch_record_property, get_batch_statuses, list_agents, list_grid_schemas, list_organizations,
    list_products, list_records, submit_batches, create_agent, update_agent,
};
//use crate::actions::{
//    do_create_agent, do_update_agent, migrations, keygen, 
//    do_create_organization, do_update_organization, 
//    do_create_product, do_update_product, 
//    do_create_schema, do_update_schema
//};
use crate::submitter::BatchSubmitter;
use actix::{Addr, SyncArbiter};
use actix_web::{
    dev,
    error::{Error as ActixError, ErrorBadRequest, ErrorInternalServerError},
    web, App, FromRequest, HttpRequest, HttpServer, Result,
};
use futures::executor::block_on;
use futures::future;
use serde::{Deserialize, Serialize};

const SYNC_ARBITER_THREAD_COUNT: usize = 2;

#[derive(Clone)]
pub struct AppState {
    batch_submitter: Box<dyn BatchSubmitter + 'static>,
    database_connection: Addr<DbExecutor>,
}

impl AppState {
    pub fn new(
        batch_submitter: Box<dyn BatchSubmitter + 'static>,
        connection_pool: ConnectionPool,
    ) -> Self {
        let database_connection = SyncArbiter::start(SYNC_ARBITER_THREAD_COUNT, move || {
            DbExecutor::new(connection_pool.clone())
        });

        AppState {
            batch_submitter,
            database_connection,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryServiceId {
    pub service_id: Option<String>,
}

pub struct AcceptServiceIdParam;

impl FromRequest for AcceptServiceIdParam {
    type Error = ActixError;
    type Future = future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let endpoint: Endpoint = if let Some(endpoint) = req.app_data::<Endpoint>() {
            endpoint.clone()
        } else {
            return future::err(ErrorInternalServerError("App state not found"));
        };

        let service_id =
            if let Ok(query) = web::Query::<QueryServiceId>::from_query(req.query_string()) {
                query.service_id.clone()
            } else {
                return future::err(ErrorBadRequest("Malformed query param"));
            };

        if service_id.is_some() && endpoint.is_sawtooth() {
            return future::err(ErrorBadRequest(
                "Circuit ID present, but dgc-platform is running in sawtooth mode",
            ));
        } else if service_id.is_none() && !endpoint.is_sawtooth() {
            return future::err(ErrorBadRequest(
                "Circuit ID is not present, but dgc-platform is running in splinter mode",
            ));
        }

        future::ok(AcceptServiceIdParam)
    }
}

pub struct RestApiShutdownHandle {
    server: dev::Server,
}

impl RestApiShutdownHandle {
    pub fn shutdown(&self) {
        block_on(self.server.stop(true));
    }
}

pub fn run(
    bind_url: &str,
    database_connection: ConnectionPool,
    batch_submitter: Box<dyn BatchSubmitter + 'static>,
    endpoint: Endpoint,
) -> Result<
    (
        RestApiShutdownHandle,
        thread::JoinHandle<Result<(), RestApiServerError>>,
    ),
    RestApiServerError,
> {
    let bind_url = bind_url.to_owned();
    let (tx, rx) = mpsc::channel();

    let join_handle = thread::Builder::new()
        .name("dgcPlatformRestApi".into())
        .spawn(move || {
            let sys = actix::System::new("dgc-platform-Rest-API");
            let state = AppState::new(batch_submitter, database_connection);

            let addr = HttpServer::new(move || {
                App::new()
                    .data(state.clone())
                    .app_data(endpoint.clone())
                    .service(web::resource("/batches").route(web::post().to(submit_batches)))
                    .service(
                        web::resource("/batch_statuses")
                            .name("batch_statuses")
                            .route(web::get().to(get_batch_statuses)),
                    )
                    //.service(web::resource("/agent").route(web::post().to(create_agent)))
                    //.service(web::resource("/agent").route(web::put().to(update_agent)))
                    .service(
                        web::scope("/agent")
                            .service(web::resource("").route(web::post().to(create_agent)))
                            .service(web::resource("").route(web::put().to(update_agent)))
                            .service(web::resource("").route(web::get().to(list_agents)))
                            .service(
                                web::resource("/{public_key}").route(web::get().to(fetch_agent)),
                            ),
                    )
                    .service(
                        web::scope("/organization")
                            .service(web::resource("").route(web::get().to(list_organizations)))
                            .service(
                                web::resource("/{id}").route(web::get().to(fetch_organization)),
                            ),
                    )
                    .service(
                        web::scope("/product")
                            .service(web::resource("").route(web::get().to(list_products)))
                            .service(web::resource("/{id}").route(web::get().to(fetch_product))),
                    )
                    .service(
                        web::scope("/schema")
                            .service(web::resource("").route(web::get().to(list_grid_schemas)))
                            .service(
                                web::resource("/{name}").route(web::get().to(fetch_grid_schema)),
                            ),
                    )
                    .service(
                        web::scope("/record")
                            .service(web::resource("").route(web::get().to(list_records)))
                            .service(
                                web::scope("/{record_id}")
                                    .service(web::resource("").route(web::get().to(fetch_record)))
                                    .service(
                                        web::resource("/property/{property_name}")
                                            .route(web::get().to(fetch_record_property)),
                                    ),
                            ),
                    )
            })
            .bind(bind_url)?
            .disable_signals()
            .system_exit()
            .run();

            tx.send(addr).map_err(|err| {
                RestApiServerError::StartUpError(format!("Unable to send Server Addr: {}", err))
            })?;
            sys.run()?;

            info!("Rest API terminating");

            Ok(())
        })?;

    let server = rx.recv().map_err(|err| {
        RestApiServerError::StartUpError(format!("Unable to receive Server Addr: {}", err))
    })?;

    Ok((RestApiShutdownHandle { server }, join_handle))
}
