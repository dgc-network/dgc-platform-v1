// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct EventProcessorError(pub String);

impl Error for EventProcessorError {}

impl fmt::Display for EventProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Event Processor Error: {}", self.0)
    }
}

#[derive(Debug)]
pub struct EventError(pub String);

impl Error for EventError {}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Event Error: {}", self.0)
    }
}

#[derive(Debug)]
pub enum EventIoError {
    ConnectionError(String),
    InvalidMessage(String),
}

impl Error for EventIoError {}

impl fmt::Display for EventIoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ConnectionError(err) => {
                write!(f, "event connection encountered an error: {}", err)
            }
            Self::InvalidMessage(err) => write!(f, "connection received invalid message: {}", err),
        }
    }
}
