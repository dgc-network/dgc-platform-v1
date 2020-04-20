// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

pub mod error;
pub mod helpers;
pub mod models;
pub mod schema;

embed_migrations!("./migrations");

use std::ops::Deref;

use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};

pub use super::database::error::DatabaseError;

pub fn create_connection_pool(database_url: &str) -> Result<ConnectionPool, DatabaseError> {
    let connection_manager = ConnectionManager::<PgConnection>::new(database_url);
    Ok(ConnectionPool {
        pool: Pool::builder()
            .build(connection_manager)
            .map_err(|err| DatabaseError::ConnectionError(Box::new(err)))?,
    })
}

pub struct Connection(PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for Connection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub struct ConnectionPool {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl ConnectionPool {
    pub fn get(&self) -> Result<Connection, DatabaseError> {
        self.pool
            .get()
            .map(Connection)
            .map_err(|err| DatabaseError::ConnectionError(Box::new(err)))
    }
}
