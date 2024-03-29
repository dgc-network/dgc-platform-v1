// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

//! Contains functions which assist with signing key management

use dirs;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use users::get_current_username;

use sawtooth_sdk::signing::secp256k1::Secp256k1PrivateKey;

use crate::error::CliError;

/// Return a signing key loaded from the user's environment
///
/// This method attempts to load the user's key from a file.  The filename
/// is constructed by appending ".priv" to the key's name.  If the name argument
/// is None, then the USER environment variable is used in its place.
///
/// The directory containing the keys is determined using the HOME
/// environment variable:
///
///   $HOME/.dgc-platform/keys/
///
/// # Arguments
///
/// * `name` - The name of the signing key, which is used to construct the
///            key's filename
///
/// # Errors
///
/// If a signing error occurs, a CliError::SigningError is returned.
///
/// If a HOME or USER environment variable is required but cannot be
/// retrieved from the environment, a CliError::VarError is returned.
pub fn load_signing_key(name: Option<String>) -> Result<Secp256k1PrivateKey, CliError> {
    let username: String = name
        .ok_or_else(|| env::var("USER"))
        .or_else(|_| {
            get_current_username()
                .ok_or(0)
                .and_then(|os_str| os_str.into_string().map_err(|_| 0))
        })
        .map_err(|_| {
            CliError::UserError(String::from(
                "Could not load signing key: unable to determine username",
            ))
        })?;

    let private_key_filename = dirs::home_dir()
        .ok_or_else(|| {
            CliError::UserError(String::from(
                "Could not load signing key: unable to determine home directory",
            ))
        })
        .and_then(|mut p| {
            p.push(".dgc-platform");
            p.push("keys");
            p.push(format!("{}.priv", &username));
            Ok(p)
        })?;

    if !private_key_filename.as_path().exists() {
        return Err(CliError::UserError(format!(
            "No such key file: {}",
            private_key_filename.display()
        )));
    }

    let mut f = File::open(&private_key_filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let key_str = match contents.lines().next() {
        Some(k) => k.trim(),
        None => {
            return Err(CliError::UserError(format!(
                "Empty key file: {}",
                private_key_filename.display()
            )));
        }
    };

    Ok(Secp256k1PrivateKey::from_hex(&key_str)?)
}
