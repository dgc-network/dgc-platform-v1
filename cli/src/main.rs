// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

#[macro_use] extern crate clap;
#[macro_use] extern crate log;
#[macro_use] extern crate diesel_migrations;
extern crate diesel;

mod actions;
mod error;
mod http;
mod key;
#[cfg(feature = "sawtooth")]
mod sawtooth;
#[cfg(feature = "splinter")]
mod splinter;
mod transaction;
mod yaml_parser;

use std::env;

use clap::ArgMatches;
use flexi_logger::{DeferredNow, LogSpecBuilder, Logger};
use grid_sdk::protocol::pike::{
    payload::{
        CreateAgentActionBuilder, CreateOrganizationActionBuilder, UpdateAgentActionBuilder,
        UpdateOrganizationActionBuilder,
    },
    state::{KeyValueEntry, KeyValueEntryBuilder},
};
use log::Record;

use crate::error::CliError;

use actions::{agents, database, keygen, organizations as orgs, products, schemas};

#[cfg(feature = "admin-keygen")]
use actions::admin;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

const DGC_PLATFORM_DAEMON_KEY: &str = "DGC_PLATFORM_DAEMON_KEY";
const DGC_PLATFORM_DAEMON_ENDPOINT: &str = "DGC_PLATFORM_DAEMON_ENDPOINT";
const DGC_PLATFORM_SERVICE_ID: &str = "DGC_PLATFORM_SERVICE_ID";

// log format for cli that will only show the log message
pub fn log_format(
    w: &mut dyn std::io::Write,
    _now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(w, "{}", record.args(),)
}

fn run() -> Result<(), CliError> {
    #[allow(unused_mut)]
    let mut app = clap_app!(myapp =>
        (name: APP_NAME)
        (version: VERSION)
        (author: "dgc.network")
        (about: "Command line for dgc-platform")
        (@arg url: --url  +takes_value "URL for the REST API")
        (@arg wait: --wait +takes_value "How long to wait for transaction to be committed")
        (@arg key: -k +takes_value "base name for private key file")
        (@arg verbose: -v +multiple +global "Log verbosely")
        (@arg quiet: -q --quiet +global conflicts_with[verbose] "Do not display output")
        (@arg service_id: --("service-id") +takes_value "The ID of the service the payload should be \
            sent to; required if running on Splinter. Format <circuit-id>::<service-id>")
        (@subcommand agent =>
            (about: "Update or create agent")
            (@setting SubcommandRequiredElseHelp)
            (@subcommand create =>
                (about: "Create an agent")
                (@arg org_id: +takes_value +required "organization ID")
                (@arg public_key: +takes_value +required "public key")
                (@arg active: --active conflicts_with[inactive] required_unless[inactive]
                    "Set agent as active")
                (@arg inactive: --inactive conflicts_with[active] required_unless[active]
                    "Set agent as inactive")
                (@arg role: --role +takes_value +use_delimiter +multiple "Roles assigned to agent")
                (@arg metadata: --metadata +takes_value +multiple +use_delimiter
                    "Key-value pairs (format: <key>=<value>) in a comma-separated list")
            )
            (@subcommand update =>
                (about: "Update an agent")
                (@arg org_id: +takes_value +required "organization ID")
                (@arg public_key: +takes_value +required "public key")
                (@arg active: --active conflicts_with[inactive] required_unless[inactive]
                    "Set agent as active")
                (@arg inactive: --inactive conflicts_with[active] required_unless[active]
                    "Set agent as inactive")
                (@arg role: --role +takes_value +use_delimiter +multiple "Roles assigned to agent")
                (@arg metadata: --metadata +takes_value +multiple +use_delimiter
                    "Key-value pairs (format: <key>=<value>) in a comma-separated list")
            )
        )
        (@subcommand organization =>
            (about: "Update or create organization")
            (@setting SubcommandRequiredElseHelp)
            (@subcommand create =>
                (about: "Create an organization")
                (@arg org_id: +required +takes_value "Unique ID for organization")
                (@arg name: +required +takes_value "Name of the organization")
                (@arg address: +takes_value "Physical address for organization")
                (@arg metadata: --metadata +takes_value +multiple +use_delimiter
                    "Key-value pairs (format: <key>=<value>) in a comma-separated list")
            )
            (@subcommand update =>
                (about: "Update an organization")
                (@arg org_id: +required +takes_value "Unique ID for organization")
                (@arg name: +required +takes_value "Name of the organization")
                (@arg address: +takes_value "Physical address for organization")
                (@arg metadata: --metadata +takes_value +multiple +use_delimiter
                    "Key-value pairs (format: <key>=<value>) in a comma-separated list")
            )
        )
        (@subcommand schema =>
            (about: "Update or create schemas")
            (@setting SubcommandRequiredElseHelp)
            (@subcommand create =>
                (about: "Create schemas from a yaml file")
                (@arg path: +takes_value +required "Path to yaml file containing a list of schema definitions")
            )
            (@subcommand update =>
                (about: "Update schemas from a yaml file")
                (@arg path: +takes_value +required "Path to yaml file containing a list of schema definitions")
            )
            (@subcommand list =>
                (about: "List currently defined schemas")
            )
            (@subcommand show =>
                (about: "Show schema specified by name argument")
                (@arg name: +takes_value +required "Name of schema")
            )
        )
        (@subcommand database =>
            (about: "Manage dgc-platform Daemon database")
            (@setting SubcommandRequiredElseHelp)
            (@subcommand migrate =>
                (about: "Run database migrations")
                (@arg database_url: --("database-url") +takes_value
                    "URL for database")
            )
        )
        (@subcommand keygen =>
           (about: "Generates keys with which the user can sign transactions and batches.")
           (@arg key_name: +takes_value "Name of the key to create")
           (@arg force: --force "Overwrite files if they exist")
           (@arg key_dir: -d --key_dir +takes_value "Specify the directory for the key files")
        )
        (@subcommand product =>
            (about: "Create, update, or delete products")
            (@setting SubcommandRequiredElseHelp)
            (@subcommand create =>
                (about: "Create products from a yaml file")
                (@arg path: +takes_value +required "Path to yaml file containing a list of products")
            )
            (@subcommand update =>
                (about: "Update products from a yaml file")
                (@arg path: +takes_value +required "Path to yaml file containing a list of products")
            )
            (@subcommand delete =>
                (about: "Delete a product")
                (@arg product_id: +required +takes_value "Unique ID for a product")
                (@arg product_type: +required +takes_value "Type of product (e.g. GS1")
            )
            (@subcommand list =>
                (about: "List currently defined products")
            )
            (@subcommand show =>
                (about: "Show product specified by ID argument")
                (@arg product_id: +takes_value +required "ID of product")
            )
        )
    );

    #[cfg(feature = "admin-keygen")]
    {
        use clap::{Arg, SubCommand};

        app = app.subcommand(
            SubCommand::with_name("admin")
                .about("Administrative commands for dgc-platform-daemon")
                .setting(clap::AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("keygen")
                        .about("Generates keys for dgc-platform-daemon to use to sign transactions and batches.")
                        .arg(
                            Arg::with_name("directory")
                                .long("dir")
                                .short("d")
                                .takes_value(true)
                                .help(
                                    "Specify the directory for the key files; \
                                     defaults to /etc/dgc-platform/keys",
                                ),
                        )
                        .arg(
                            Arg::with_name("force")
                                .long("force")
                                .conflicts_with("skip")
                                .help("Overwrite files if they exist"),
                        )
                        .arg(
                            Arg::with_name("skip")
                                .long("skip")
                                .conflicts_with("force")
                                .help("Check if files exist; generate if missing"),
                        ),
                ),
        );
    }

    let matches = app.get_matches();

    let log_level = if matches.is_present("quiet") {
        log::LevelFilter::Error
    } else {
        match matches.occurrences_of("verbose") {
            0 => log::LevelFilter::Info,
            1 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        }
    };
    let mut log_spec_builder = LogSpecBuilder::new();
    log_spec_builder.default(log_level);

    Logger::with(log_spec_builder.build())
        .format(log_format)
        .start()?;

    let url = matches
        .value_of("url")
        .map(String::from)
        .or_else(|| env::var(DGC_PLATFORM_DAEMON_ENDPOINT).ok())
        .unwrap_or_else(|| String::from("http://localhost:8000"));

    let key = matches
        .value_of("key")
        .map(String::from)
        .or_else(|| env::var(DGC_PLATFORM_DAEMON_KEY).ok());

    let wait = value_t!(matches, "wait", u64).unwrap_or(0);

    let service_id = matches
        .value_of("service_id")
        .map(String::from)
        .or_else(|| env::var(DGC_PLATFORM_SERVICE_ID).ok());

    match matches.subcommand() {
        #[cfg(feature = "admin-keygen")]
        ("admin", Some(m)) => match m.subcommand() {
            ("keygen", Some(m)) => {
                let conflict_strategy = if m.is_present("force") {
                    admin::ConflictStrategy::Force
                } else if m.is_present("skip") {
                    admin::ConflictStrategy::Skip
                } else {
                    admin::ConflictStrategy::Error
                };

                admin::do_keygen(m.value_of("directory"), conflict_strategy)?;
            }
            _ => unreachable!(),
        },
        ("agent", Some(m)) => match m.subcommand() {
            ("create", Some(m)) => {
                let active = if m.is_present("inactive") {
                    false
                } else if m.is_present("active") {
                    true
                } else {
                    return Err(CliError::UserError(
                        "--active or --inactive flag must be provided".to_string(),
                    ));
                };
                let create_agent = CreateAgentActionBuilder::new()
                    .with_org_id(m.value_of("org_id").unwrap().into())
                    .with_public_key(m.value_of("public_key").unwrap().into())
                    .with_active(active)
                    .with_roles(
                        m.values_of("role")
                            .unwrap_or_default()
                            .map(String::from)
                            .collect::<Vec<String>>(),
                    )
                    .with_metadata(parse_metadata(&m)?)
                    .build()
                    .map_err(|err| CliError::UserError(format!("{}", err)))?;

                agents::do_create_agent(&url, key, wait, create_agent, service_id)?
            }
            ("update", Some(m)) => {
                let active = if m.is_present("inactive") {
                    false
                } else if m.is_present("active") {
                    true
                } else {
                    return Err(CliError::UserError(
                        "--active or --inactive flag must be provided".to_string(),
                    ));
                };
                let update_agent = UpdateAgentActionBuilder::new()
                    .with_org_id(m.value_of("org_id").unwrap().into())
                    .with_public_key(m.value_of("public_key").unwrap().into())
                    .with_active(active)
                    .with_roles(
                        m.values_of("role")
                            .unwrap_or_default()
                            .map(String::from)
                            .collect::<Vec<String>>(),
                    )
                    .with_metadata(parse_metadata(&m)?)
                    .build()
                    .map_err(|err| CliError::UserError(format!("{}", err)))?;

                agents::do_update_agent(&url, key, wait, update_agent, service_id)?
            }
            _ => return Err(CliError::UserError("Subcommand not recognized".into())),
        },
        ("organization", Some(m)) => match m.subcommand() {
            ("create", Some(m)) => {
                let create_org = CreateOrganizationActionBuilder::new()
                    .with_org_id(m.value_of("org_id").unwrap().into())
                    .with_name(m.value_of("name").unwrap().into())
                    .with_address(m.value_of("address").unwrap().into())
                    .with_metadata(parse_metadata(&m)?)
                    .build()
                    .map_err(|err| CliError::UserError(format!("{}", err)))?;

                orgs::do_create_organization(&url, key, wait, create_org, service_id)?
            }
            ("update", Some(m)) => {
                let update_org = UpdateOrganizationActionBuilder::new()
                    .with_org_id(m.value_of("org_id").unwrap().into())
                    .with_name(m.value_of("name").unwrap().into())
                    .with_address(m.value_of("address").unwrap().into())
                    .with_metadata(parse_metadata(&m)?)
                    .build()
                    .map_err(|err| CliError::UserError(format!("{}", err)))?;

                orgs::do_update_organization(&url, key, wait, update_org, service_id)?
            }
            _ => return Err(CliError::UserError("Subcommand not recognized".into())),
        },
        ("schema", Some(m)) => match m.subcommand() {
            ("create", Some(m)) => schemas::do_create_schemas(
                &url,
                key,
                wait,
                m.value_of("path").unwrap(),
                service_id,
            )?,
            ("update", Some(m)) => schemas::do_update_schemas(
                &url,
                key,
                wait,
                m.value_of("path").unwrap(),
                service_id,
            )?,
            ("list", Some(_)) => schemas::do_list_schemas(&url, service_id)?,
            ("show", Some(m)) => {
                schemas::do_show_schema(&url, m.value_of("name").unwrap(), service_id)?
            }
            _ => return Err(CliError::UserError("Subcommand not recognized".into())),
        },
        ("database", Some(m)) => match m.subcommand() {
            ("migrate", Some(m)) => database::run_migrations(
                m.value_of("database_url")
                    .unwrap_or("postgres://dgc-platform:dgc_platform_example@localhost/dgc-platform"),
            )?,
            _ => return Err(CliError::UserError("Subcommand not recognized".into())),
        },
        ("keygen", Some(m)) => keygen::generate_keys(
            m.value_of("key_name"),
            m.is_present("force"),
            m.value_of("key_dir"),
        )?,
        ("product", Some(m)) => match m.subcommand() {
            ("create", Some(m)) => products::do_create_products(
                &url,
                key,
                wait,
                m.value_of("path").unwrap(),
                service_id,
            )?,
            ("update", Some(m)) => products::do_update_products(
                &url,
                key,
                wait,
                m.value_of("path").unwrap(),
                service_id,
            )?,
            ("delete", Some(m)) => products::do_delete_products(
                &url,
                key,
                wait,
                m.value_of("product_id").unwrap(),
                m.value_of("product_type").unwrap(),
                service_id,
            )?,
            ("list", Some(_)) => products::do_list_products(&url, service_id)?,
            ("show", Some(m)) => {
                products::do_show_products(&url, m.value_of("product_id").unwrap(), service_id)?
            }
            _ => return Err(CliError::UserError("Subcommand not recognized".into())),
        },
        _ => return Err(CliError::UserError("Subcommand not recognized".into())),
    }

    Ok(())
}

fn parse_metadata(matches: &ArgMatches) -> Result<Vec<KeyValueEntry>, CliError> {
    let metadata = matches
        .values_of("metadata")
        .unwrap_or_default()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut key_value_entries = Vec::new();

    for data in metadata {
        let entries = data.split('=').map(String::from).collect::<Vec<String>>();

        let (key, value) = if entries.len() != 2 {
            return Err(CliError::UserError(format!("Metadata malformed: {}", data)));
        } else {
            (entries[0].clone(), entries[1].clone())
        };

        key_value_entries.push(
            KeyValueEntryBuilder::new()
                .with_key(key)
                .with_value(value)
                .build()
                .map_err(|err| CliError::UserError(format!("Metadata malformed: {}", err)))?,
        );
    }

    Ok(key_value_entries)
}

fn main() {
    if let Err(e) = run() {
        error!("{}", e);
        std::process::exit(1);
    }
}
