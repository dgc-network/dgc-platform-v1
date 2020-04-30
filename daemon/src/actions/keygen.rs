// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use std::fs::{self, OpenOptions};
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;

use sawtooth_sdk::signing;
use users::get_current_username;

use crate::error::CliError;

/// Generates a public/private key pair that can be used to sign transactions.
/// If no directory is provided, the keys are created in the default directory
///
///   $HOME/.dgc-platform/keys/
///
/// If no key_name is provided the key name is set to USER environment variable.
pub fn generate_keys(
    key_name: Option<&str>,
    force: bool,
    key_directory: Option<&str>,
) -> Result<(), CliError> {
    let key_name = match key_name {
        Some(name) => name.to_string(),
        None => get_current_username()
            .ok_or(0)
            .and_then(|os_str| os_str.into_string().map_err(|_| 0))
            .map_err(|_| {
                CliError::UserError(String::from(
                    "Could not determine key name, please provide one.",
                ))
            })?,
    };

    let key_dir = match key_directory {
        Some(path) => {
            let dir = PathBuf::from(&path);
            if !dir.exists() {
                return Err(CliError::UserError(format!("No such directory: {}", path)));
            }
            dir
        }
        None => {
            let key_path = dirs::home_dir()
                .ok_or_else(|| {
                    CliError::UserError(String::from("Unable to determine home directory"))
                })
                .and_then(|mut p| {
                    p.push(".dgc-platform");
                    p.push("keys");
                    Ok(p)
                })?;
            if !key_path.exists() {
                fs::create_dir_all(key_path.clone())?;
            }
            key_path
        }
    };

    let mut public_key_path = key_dir.clone();
    public_key_path.push(format!("{}.pub", &key_name));
    let mut private_key_path = key_dir.clone();
    private_key_path.push(format!("{}.priv", &key_name));

    if (public_key_path.exists() || private_key_path.exists()) && !force {
        return Err(CliError::UserError(format!(
            "Key files already exist at {:?}. Rerun with --force to overwrite existing files",
            key_dir
        )));
    }

    let context = signing::create_context("secp256k1")?;

    let private_key = context.new_random_private_key()?;

    let public_key = context.get_public_key(&*private_key)?;

    if public_key_path.exists() {
        info!("Overwriting file: {:?}", public_key_path);
    } else {
        info!("Writing file: {:?}", public_key_path);
    }
    let public_key_file = OpenOptions::new()
        .write(true)
        .create(true)
        .mode(0o644)
        .open(public_key_path.as_path())?;

    writeln!(&public_key_file, "{}", public_key.as_hex())?;

    if private_key_path.exists() {
        info!("Overwriting file: {:?}", private_key_path);
    } else {
        info!("Writing file: {:?}", private_key_path);
    }
    let private_key_file = OpenOptions::new()
        .write(true)
        .create(true)
        .mode(0o640)
        .open(private_key_path.as_path())?;

    writeln!(&private_key_file, "{}", &private_key.as_hex())?;

    Ok(())
}
