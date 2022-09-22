/* -------------------------------------------------------------------------- *\
 *             Apache 2.0 License Copyright © 2022 The Aurae Authors          *
 *                                                                            *
 *                +--------------------------------------------+              *
 *                |   █████╗ ██╗   ██╗██████╗  █████╗ ███████╗ |              *
 *                |  ██╔══██╗██║   ██║██╔══██╗██╔══██╗██╔════╝ |              *
 *                |  ███████║██║   ██║██████╔╝███████║█████╗   |              *
 *                |  ██╔══██║██║   ██║██╔══██╗██╔══██║██╔══╝   |              *
 *                |  ██║  ██║╚██████╔╝██║  ██║██║  ██║███████╗ |              *
 *                |  ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝ |              *
 *                +--------------------------------------------+              *
 *                                                                            *
 *                         Distributed Systems Runtime                        *
 *                                                                            *
 * -------------------------------------------------------------------------- *
 *                                                                            *
 *   Licensed under the Apache License, Version 2.0 (the "License");          *
 *   you may not use this file except in compliance with the License.         *
 *   You may obtain a copy of the License at                                  *
 *                                                                            *
 *       http://www.apache.org/licenses/LICENSE-2.0                           *
 *                                                                            *
 *   Unless required by applicable law or agreed to in writing, software      *
 *   distributed under the License is distributed on an "AS IS" BASIS,        *
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. *
 *   See the License for the specific language governing permissions and      *
 *   limitations under the License.                                           *
 *                                                                            *
\* -------------------------------------------------------------------------- */

#![warn(clippy::unwrap_used)]

use auraed::*;
use clap::Parser;
use log::*;
use std::path::PathBuf;
use syslog::{BasicLogger, Facility, Formatter3164};
//use futures::Stream;
//use std::{error::Error, io::ErrorKind, net::ToSocketAddrs, path::Path, pin::Pin, time::Duration};
//use tokio::sync::mpsc;
//use tokio_stream::{wrappers::ReceiverStream, StreamExt};
//use tonic::{transport::Server, Request, Response, Status, Streaming};

const EXIT_OKAY: i32 = 0;
const EXIT_ERROR: i32 = 1;
const AURAED_SYSLOG_NAME: &str = "auraed";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct AuraedOptions {
    #[clap(
        long,
        value_parser,
        default_value = "/etc/aurae/pki/_signed.server.crt"
    )]
    server_crt: String,

    #[clap(long, value_parser, default_value = "/etc/aurae/pki/server.key")]
    server_key: String,

    #[clap(long, value_parser, default_value = "/etc/aurae/pki/ca.crt")]
    ca_crt: String,

    #[clap(short, long, value_parser, default_value = auraed::AURAE_SOCK)]
    socket: String,

    #[clap(short, long)]
    verbose: bool,
}

async fn daemon() -> i32 {
    let options = AuraedOptions::parse();

    // The logger will log to stdout and the syslog by default.
    // We hold the opinion that the program is either "verbose"
    // or it's not.
    //
    // Normal mode: Info, Warn, Error
    // Verbose mode: Debug, Trace, Info, Warn, Error
    // let logger_level = if matches.is_present("verbose") {
    let logger_level = if options.verbose { Level::Trace } else { Level::Info };

    // Syslog formatter
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: AURAED_SYSLOG_NAME.into(),
        pid: 0,
    };

    // Initialize the logger
    let logger_simple = simplelog::SimpleLogger::new(
        logger_level.to_level_filter(),
        simplelog::Config::default(),
    );
    let logger_syslog = syslog::unix(formatter).unwrap();
    match multi_log::MultiLogger::init(
        vec![logger_simple, Box::new(BasicLogger::new(logger_syslog))],
        logger_level,
    ) {
        Ok(_) => {}
        Err(e) => panic!("unable to connect to syslog: {:?}", e),
    };

    trace!("**Logging: Verbose Mode**");
    info!("Starting Aurae Daemon Runtime...");

    // Load Variables
    //let key = matches.value_of("key").unwrap();
    //let sock = matches.value_of("sock").unwrap();

    let runtime = AuraedRuntime {
        server_crt: PathBuf::from(options.server_crt),
        server_key: PathBuf::from(options.server_key),
        ca_crt: PathBuf::from(options.ca_crt),
        socket: PathBuf::from(options.socket),
    };

    let e = runtime.run().await;
    if e.is_err() {
        error!("{:?}", e);
    }

    // Return
    if e.is_err() {
        EXIT_ERROR
    } else {
        EXIT_OKAY
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exit_code = daemon();
    std::process::exit(exit_code.await);
}
