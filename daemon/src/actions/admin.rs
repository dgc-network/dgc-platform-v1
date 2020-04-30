// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;

use sawtooth_sdk::signing;

//use crate::error::CliError;
use crate::actions::error::CliError;

const DEFAULT_KEY_DIR: &str = "/etc/dgc-platform/keys";

pub enum ConflictStrategy {
    Force,
    Skip,
    Error,
}

pub fn do_keygen(
    directory: Option<&str>,
    conflict_strategy: ConflictStrategy,
) -> Result<(), CliError> {
    let key_dir = match directory {
        Some(key_dir) => {
            if !PathBuf::from(key_dir).exists() {
                return Err(CliError::UserError(format!("{} does not exist", key_dir)));
            }
            key_dir
        }
        None => {
            if !PathBuf::from(DEFAULT_KEY_DIR).exists() {
                return Err(CliError::UserError(format!(
                    "{} does not exist; verify that you have dgc-platform-daemon installed on this system",
                    DEFAULT_KEY_DIR
                )));
            }
            DEFAULT_KEY_DIR
        }
    };

    let public_key_path: PathBuf = [key_dir, "dgc-platform-daemon.pub"].iter().collect();
    let private_key_path: PathBuf = [key_dir, "dgc-platform-daemon.priv"].iter().collect();

    match conflict_strategy {
        ConflictStrategy::Force => (),
        ConflictStrategy::Skip => {
            if public_key_path.exists() && !private_key_path.exists() {
                return Err(CliError::UserError(format!(
                    "{} already exists without a corresponding private key. \
                     Rerun with --force to overwrite existing files",
                    public_key_path.as_path().display(),
                )));
            }

            if !public_key_path.exists() && private_key_path.exists() {
                return Err(CliError::UserError(format!(
                    "{} already exists without a corresponding public key. \
                     Rerun with --force to overwrite existing files",
                    private_key_path.as_path().display(),
                )));
            }

            if public_key_path.exists() && private_key_path.exists() {
                debug!("Admin keys exist; skipping generation");
                return Ok(());
            }
        }
        ConflictStrategy::Error => {
            if public_key_path.exists() || private_key_path.exists() {
                return Err(CliError::UserError(format!(
                    "Key files already exist at {}. Rerun with --force to \
                     overwrite existing files",
                    key_dir
                )));
            }
        }
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

    writeln!(&public_key_file, "{}", &public_key.as_hex())?;

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
