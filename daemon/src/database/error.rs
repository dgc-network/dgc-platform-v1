// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError(Box<dyn Error>),
    MigrationError(Box<dyn Error>),
    QueryError(Box<dyn Error>),
}

impl Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DatabaseError::ConnectionError(e) => Some(&**e),
            DatabaseError::MigrationError(e) => Some(&**e),
            DatabaseError::QueryError(e) => Some(&**e),
        }
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseError::ConnectionError(e) => write!(f, "Unable to connect to database: {}", e),
            DatabaseError::MigrationError(e) => write!(f, "Unable to migrate database: {}", e),
            DatabaseError::QueryError(e) => write!(f, "Database query failed: {}", e),
        }
    }
}

impl From<diesel::ConnectionError> for DatabaseError {
    fn from(err: diesel::ConnectionError) -> Self {
        DatabaseError::ConnectionError(Box::new(err))
    }
}

impl From<diesel_migrations::RunMigrationsError> for DatabaseError {
    fn from(err: diesel_migrations::RunMigrationsError) -> Self {
        DatabaseError::MigrationError(Box::new(err))
    }
}

impl From<diesel::result::Error> for DatabaseError {
    fn from(err: diesel::result::Error) -> Self {
        DatabaseError::QueryError(Box::new(err))
    }
}
