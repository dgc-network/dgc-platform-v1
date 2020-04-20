// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug)]
pub enum CliError {
    LoggingInitializationError(Box<flexi_logger::FlexiLoggerError>),
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CliError::LoggingInitializationError(e) => {
                write!(f, "Logging initialization error: {}", e)
            }
        }
    }
}

impl From<flexi_logger::FlexiLoggerError> for CliError {
    fn from(err: flexi_logger::FlexiLoggerError) -> CliError {
        CliError::LoggingInitializationError(Box::new(err))
    }
}
