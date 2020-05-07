// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use crate::database::DatabaseError;
//use crate::actions::error::CliError::UserError;

use actix::MailboxError;
use actix_web::error::{PayloadError, UrlGenerationError};
use actix_web::{
    error::{Error as ActixError, ResponseError},
    HttpResponse,
};
use diesel;
use futures::future::{Future, TryFutureExt};

use std;
use std::error::Error as StdError;
//use std::error::Error;
use std::fmt;
use std::io;

use grid_sdk::protos;
use protobuf;
use sabre_sdk;
use sawtooth_sdk::signing;

#[derive(Debug)]
pub enum RestApiServerError {
    StartUpError(String),
    StdError(std::io::Error),
}

impl From<std::io::Error> for RestApiServerError {
    fn from(err: std::io::Error) -> RestApiServerError {
        RestApiServerError::StdError(err)
    }
}

impl StdError for RestApiServerError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            RestApiServerError::StartUpError(_) => None,
            RestApiServerError::StdError(err) => Some(err),
        }
    }
}

impl fmt::Display for RestApiServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RestApiServerError::StartUpError(e) => write!(f, "Start-up Error: {}", e),
            RestApiServerError::StdError(e) => write!(f, "Std Error: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum RestApiResponseError {
    BadRequest(String),
    SawtoothConnectionError(String),
    SawtoothValidatorResponseError(String),
    RequestHandlerError(String),
    DatabaseError(String),
    NotFoundError(String),
    UserError(String),
    SigningError(signing::Error),
    IoError(io::Error),
    ProtobufError(protobuf::ProtobufError),
    GridProtoError(protos::ProtoConversionError),
    SabreProtoError(sabre_sdk::protos::ProtoConversionError),
}

impl StdError for RestApiResponseError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            RestApiResponseError::BadRequest(_) => None,
            RestApiResponseError::SawtoothConnectionError(_) => None,
            RestApiResponseError::SawtoothValidatorResponseError(_) => None,
            RestApiResponseError::RequestHandlerError(_) => None,
            RestApiResponseError::DatabaseError(_) => None,
            RestApiResponseError::NotFoundError(_) => None,
            RestApiResponseError::UserError(_) => None,
            RestApiResponseError::SigningError(err) => Some(err),
            RestApiResponseError::IoError(err) => Some(err),
            RestApiResponseError::ProtobufError(err) => Some(err),
            RestApiResponseError::GridProtoError(err) => Some(err),
            RestApiResponseError::SabreProtoError(err) => Some(err),
        }
    }
}

impl fmt::Display for RestApiResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RestApiResponseError::BadRequest(ref s) => write!(f, "Bad Request: {}", s),
            RestApiResponseError::SawtoothConnectionError(ref s) => {
                write!(f, "Zmq Connection Error: {}", s)
            }
            RestApiResponseError::SawtoothValidatorResponseError(ref s) => {
                write!(f, "Sawtooth Validator Response Error: {}", s)
            }
            RestApiResponseError::RequestHandlerError(ref s) => {
                write!(f, "Request Handler Error: {}", s)
            }
            RestApiResponseError::NotFoundError(ref s) => write!(f, "Not Found Error: {}", s),
            RestApiResponseError::DatabaseError(ref s) => write!(f, "Database Error: {}", s),
            RestApiResponseError::UserError(ref err) => write!(f, "Error: {}", err),
            RestApiResponseError::SigningError(ref err) => write!(f, "SigningError: {}", err),
            RestApiResponseError::IoError(ref err) => write!(f, "IoError: {}", err),
            RestApiResponseError::ProtobufError(ref err) => write!(f, "ProtobufError: {}", err),
            RestApiResponseError::GridProtoError(ref err) => write!(f, "dgc-platform Proto Error: {}", err),
            RestApiResponseError::SabreProtoError(ref err) => write!(f, "Sabre Proto Error: {}", err),
        }
    }
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl RestApiResponseError {
    pub fn future_box(self) -> Box<dyn Future<Output = Result<HttpResponse, ActixError>>> {
        match self {
            RestApiResponseError::BadRequest(ref message) => {
                Box::new(HttpResponse::BadRequest().json(message).into_future())
            }
            RestApiResponseError::SawtoothConnectionError(ref message) => Box::new(
                HttpResponse::ServiceUnavailable()
                    .json(message)
                    .into_future(),
            ),
            RestApiResponseError::DatabaseError(ref message) => Box::new(
                HttpResponse::ServiceUnavailable()
                    .json(message)
                    .into_future(),
            ),
            //RestApiResponseError::UserError(ref message) => Box::new(
            //    HttpResponse::ServiceUnavailable()
            //        .json(message)
            //        .into_future(),
            //),
            RestApiResponseError::NotFoundError(ref message) => {
                Box::new(HttpResponse::NotFound().json(message).into_future())
            }
            _ => Box::new(
                HttpResponse::InternalServerError()
                    .json("Internal Server Error")
                    .into_future(),
            ),
        }
    }
}
// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for RestApiResponseError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            RestApiResponseError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(message)
            }
            RestApiResponseError::SawtoothConnectionError(ref message) => {
                HttpResponse::ServiceUnavailable().json(message)
            }
            RestApiResponseError::DatabaseError(ref message) => {
                HttpResponse::ServiceUnavailable().json(message)
            }
            //RestApiResponseError::UserError(ref message) => {
            //    HttpResponse::ServiceUnavailable().json(message)
            //}
            RestApiResponseError::NotFoundError(ref message) => {
                HttpResponse::NotFound().json(message)
            }
            _ => HttpResponse::InternalServerError().json("Internal Server Error"),
        }
    }
}

impl From<PayloadError> for RestApiResponseError {
    fn from(err: PayloadError) -> RestApiResponseError {
        RestApiResponseError::BadRequest(format!(
            "Payload was not well formated. {}",
            err.to_string()
        ))
    }
}

impl From<MailboxError> for RestApiResponseError {
    fn from(err: MailboxError) -> RestApiResponseError {
        RestApiResponseError::RequestHandlerError(format!(
            "Failed to deliver message to request handler. {}",
            err.to_string()
        ))
    }
}

impl From<UrlGenerationError> for RestApiResponseError {
    fn from(err: UrlGenerationError) -> RestApiResponseError {
        RestApiResponseError::RequestHandlerError(format!(
            "Failed generate response URL. {}",
            err.to_string()
        ))
    }
}

impl From<DatabaseError> for RestApiResponseError {
    fn from(err: DatabaseError) -> RestApiResponseError {
        RestApiResponseError::DatabaseError(format!(
            "Database Error occured: {}", 
            err.to_string()
        ))
    }
}

impl From<diesel::result::Error> for RestApiResponseError {
    fn from(err: diesel::result::Error) -> Self {
        RestApiResponseError::DatabaseError(format!(
            "Database Result Error occured: {}",
            err.to_string()
        ))
    }
}

impl From<signing::Error> for RestApiResponseError {
    fn from(err: signing::Error) -> Self {
        RestApiResponseError::SigningError(err)
    }
}

impl From<io::Error> for RestApiResponseError {
    fn from(err: io::Error) -> Self {
        RestApiResponseError::IoError(err)
    }
}

impl From<protobuf::ProtobufError> for RestApiResponseError {
    fn from(err: protobuf::ProtobufError) -> Self {
        RestApiResponseError::ProtobufError(err)
    }
}

impl From<protos::ProtoConversionError> for RestApiResponseError {
    fn from(err: protos::ProtoConversionError) -> Self {
        RestApiResponseError::GridProtoError(err)
    }
}

impl From<sabre_sdk::protos::ProtoConversionError> for RestApiResponseError {
    fn from(err: sabre_sdk::protos::ProtoConversionError) -> Self {
        RestApiResponseError::SabreProtoError(err)
    }
}
/*
impl From<crate::rest_api::error::RestApiResponseError> for RestApiResponseError {
    fn from(err: crate::rest_api::error::RestApiResponseError) -> RestApiResponseError {
        RestApiResponseError::UserError(format!(
            "User Error occured: {}", 
            err.to_string()
        ))
    }
}
*/