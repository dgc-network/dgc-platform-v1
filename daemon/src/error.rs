// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use std::error::Error;
use std::fmt;

use crate::database::DatabaseError;
use crate::event::EventProcessorError;
use crate::rest_api::RestApiServerError;
#[cfg(feature = "splinter-support")]
use crate::splinter::app_auth_handler::error::AppAuthHandlerError;

use grid_sdk::protos;
use protobuf;
use reqwest;
use sabre_sdk;
use sawtooth_sdk::signing;
use serde_yaml;
use std;
use std::error::Error as StdError;
use std::io;

#[derive(Debug)]
pub enum DaemonError {
    DatabaseError(Box<dyn Error>),
    LoggingInitializationError(Box<flexi_logger::FlexiLoggerError>),
    ConfigurationError(Box<ConfigurationError>),
    EventProcessorError(Box<EventProcessorError>),
    RestApiError(RestApiServerError),
    StartUpError(Box<dyn Error>),
    ShutdownError(String),
    UnsupportedEndpoint(String),
    #[cfg(feature = "splinter-support")]
    AppAuthHandlerError(AppAuthHandlerError),
}

impl Error for DaemonError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DaemonError::DatabaseError(err) => Some(&**err),
            DaemonError::LoggingInitializationError(err) => Some(err),
            DaemonError::ConfigurationError(err) => Some(err),
            DaemonError::EventProcessorError(err) => Some(err),
            DaemonError::RestApiError(err) => Some(err),
            DaemonError::StartUpError(err) => Some(&**err),
            DaemonError::ShutdownError(_) => None,
            DaemonError::UnsupportedEndpoint(_) => None,
            #[cfg(feature = "splinter-support")]
            DaemonError::AppAuthHandlerError(err) => Some(err),
        }
    }
}

impl fmt::Display for DaemonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DaemonError::DatabaseError(e) => write!(f, "Database Error: {}", e),
            DaemonError::LoggingInitializationError(e) => {
                write!(f, "Logging initialization error: {}", e)
            }
            DaemonError::ConfigurationError(e) => write!(f, "Configuration error: {}", e),
            DaemonError::EventProcessorError(e) => write!(f, "Event Processor Error: {}", e),
            DaemonError::RestApiError(e) => write!(f, "Rest API error: {}", e),
            DaemonError::StartUpError(e) => write!(f, "Start-up error: {}", e),
            DaemonError::ShutdownError(msg) => write!(f, "Unable to cleanly shutdown: {}", msg),
            DaemonError::UnsupportedEndpoint(msg) => write!(f, "{}", msg),
            #[cfg(feature = "splinter-support")]
            DaemonError::AppAuthHandlerError(e) => {
                write!(f, "Application Authorization Handler Error: {}", e)
            }
        }
    }
}

impl From<flexi_logger::FlexiLoggerError> for DaemonError {
    fn from(err: flexi_logger::FlexiLoggerError) -> DaemonError {
        DaemonError::LoggingInitializationError(Box::new(err))
    }
}

#[derive(Debug, PartialEq)]
pub enum ConfigurationError {
    MissingValue(String),
}

impl Error for ConfigurationError {}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigurationError::MissingValue(config_field_name) => {
                write!(f, "Missing configuration for {}", config_field_name)
            }
        }
    }
}

impl From<ConfigurationError> for DaemonError {
    fn from(err: ConfigurationError) -> Self {
        DaemonError::ConfigurationError(Box::new(err))
    }
}

impl From<RestApiServerError> for DaemonError {
    fn from(err: RestApiServerError) -> DaemonError {
        DaemonError::RestApiError(err)
    }
}

impl From<EventProcessorError> for DaemonError {
    fn from(err: EventProcessorError) -> Self {
        DaemonError::EventProcessorError(Box::new(err))
    }
}

impl From<DatabaseError> for DaemonError {
    fn from(err: DatabaseError) -> Self {
        DaemonError::DatabaseError(Box::new(err))
    }
}

#[cfg(feature = "splinter-support")]
impl From<AppAuthHandlerError> for DaemonError {
    fn from(err: AppAuthHandlerError) -> DaemonError {
        DaemonError::AppAuthHandlerError(err)
    }
}


#[derive(Debug)]
pub enum CliError {
    LoggingInitializationError(Box<flexi_logger::FlexiLoggerError>),
    InvalidYamlError(String),
    PayloadError(String),
    UserError(String),
    SigningError(signing::Error),
    DatabaseError(String),
    IoError(io::Error),
    ProtobufError(protobuf::ProtobufError),
    ReqwestError(reqwest::Error),
    GridProtoError(protos::ProtoConversionError),
    SabreProtoError(sabre_sdk::protos::ProtoConversionError),
}

impl StdError for CliError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            CliError::LoggingInitializationError(err) => Some(err),
            CliError::InvalidYamlError(_) => None,
            CliError::PayloadError(_) => None,
            CliError::UserError(_) => None,
            CliError::DatabaseError(_) => None,
            CliError::IoError(err) => Some(err),
            CliError::ProtobufError(err) => Some(err),
            CliError::SigningError(err) => Some(err),
            CliError::ReqwestError(err) => Some(err),
            CliError::GridProtoError(err) => Some(err),
            CliError::SabreProtoError(err) => Some(err),
        }
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CliError::UserError(ref err) => write!(f, "Error: {}", err),
            CliError::InvalidYamlError(ref err) => write!(f, "InvalidYamlError: {}", err),
            CliError::PayloadError(ref err) => write!(f, "PayloadError: {}", err),
            CliError::IoError(ref err) => write!(f, "IoError: {}", err),
            CliError::DatabaseError(ref err) => write!(f, "DatabaseError: {}", err),
            CliError::SigningError(ref err) => write!(f, "SigningError: {}", err),
            CliError::ProtobufError(ref err) => write!(f, "ProtobufError: {}", err),
            CliError::LoggingInitializationError(ref err) => {
                write!(f, "LoggingInitializationError: {}", err)
            }
            CliError::ReqwestError(ref err) => write!(f, "Reqwest Error: {}", err),
            CliError::GridProtoError(ref err) => write!(f, "dgc-platform Proto Error: {}", err),
            CliError::SabreProtoError(ref err) => write!(f, "Sabre Proto Error: {}", err),
        }
    }
}

impl From<flexi_logger::FlexiLoggerError> for CliError {
    fn from(err: flexi_logger::FlexiLoggerError) -> Self {
        CliError::LoggingInitializationError(Box::new(err))
    }
}

impl From<signing::Error> for CliError {
    fn from(err: signing::Error) -> Self {
        CliError::SigningError(err)
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> Self {
        CliError::IoError(err)
    }
}
impl From<serde_yaml::Error> for CliError {
    fn from(err: serde_yaml::Error) -> Self {
        CliError::InvalidYamlError(err.to_string())
    }
}

impl From<protobuf::ProtobufError> for CliError {
    fn from(err: protobuf::ProtobufError) -> Self {
        CliError::ProtobufError(err)
    }
}

impl From<reqwest::Error> for CliError {
    fn from(err: reqwest::Error) -> Self {
        CliError::ReqwestError(err)
    }
}

impl From<protos::ProtoConversionError> for CliError {
    fn from(err: protos::ProtoConversionError) -> Self {
        CliError::GridProtoError(err)
    }
}

impl From<sabre_sdk::protos::ProtoConversionError> for CliError {
    fn from(err: sabre_sdk::protos::ProtoConversionError) -> Self {
        CliError::SabreProtoError(err)
    }
}
