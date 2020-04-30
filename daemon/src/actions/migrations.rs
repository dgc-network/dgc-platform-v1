// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use crate::error::CliError;

use diesel::{connection::Connection as _, pg::PgConnection};

embed_migrations!("../daemon/migrations");

pub fn run_migrations(database_url: &str) -> Result<(), CliError> {
    let connection = PgConnection::establish(database_url)
        .map_err(|err| CliError::DatabaseError(err.to_string()))?;

    embedded_migrations::run(&connection)
        .map_err(|err| CliError::DatabaseError(err.to_string()))?;

    info!("Successfully applied migrations");

    Ok(())
}
