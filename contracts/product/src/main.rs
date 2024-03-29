// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

#[macro_use]
extern crate cfg_if;
cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        #[macro_use]
        extern crate clap;
        #[macro_use]
        extern crate log;
        use std::process;
        use log::LogLevelFilter;
        use log4rs::append::console::ConsoleAppender;
        use log4rs::config::{Appender, Config, Root};
        use log4rs::encode::pattern::PatternEncoder;
        use sawtooth_sdk::processor::TransactionProcessor;
        use crate::handler::ProductTransactionHandler;
    } else {
        #[macro_use]
        extern crate sabre_sdk;
    }
}

mod addressing;
pub mod handler;
mod payload;
mod state;
mod validation;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let matches = clap_app!(intkey =>
        (version: crate_version!())
        (about: "dgc-platform Product Processor (Rust)")
        (@arg connect: -C --connect +takes_value
         "connection endpoint for validator")
        (@arg verbose: -v --verbose +multiple
         "increase output verbosity"))
    .get_matches();

    let endpoint = matches
        .value_of("connect")
        .unwrap_or("tcp://localhost:4004");

    let console_log_level;
    match matches.occurrences_of("verbose") {
        0 => console_log_level = LogLevelFilter::Warn,
        1 => console_log_level = LogLevelFilter::Info,
        2 => console_log_level = LogLevelFilter::Debug,
        _ => console_log_level = LogLevelFilter::Trace,
    }

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({l:5.5})} | {({M}:{L}):20.20} | {m}{n}",
        )))
        .build();

    let config = match Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(console_log_level))
    {
        Ok(x) => x,
        Err(_) => process::exit(1),
    };

    match log4rs::init_config(config) {
        Ok(_) => (),
        Err(_) => process::exit(1),
    }

    let handler = ProductTransactionHandler::new();
    let mut processor = TransactionProcessor::new(endpoint);

    info!("Console logging level: {}", console_log_level);

    processor.add_handler(&handler);
    processor.start();
}

#[cfg(target_arch = "wasm32")]
fn main() {}
