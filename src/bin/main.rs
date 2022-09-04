/* ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ *\
 *             Apache 2.0 License Copyright © 2022 The Aurae Authors          *
 *                                                                            *
 *                ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓              *
 *                ┃   █████╗ ██╗   ██╗██████╗  █████╗ ███████╗ ┃              *
 *                ┃  ██╔══██╗██║   ██║██╔══██╗██╔══██╗██╔════╝ ┃              *
 *                ┃  ███████║██║   ██║██████╔╝███████║█████╗   ┃              *
 *                ┃  ██╔══██║██║   ██║██╔══██╗██╔══██║██╔══╝   ┃              *
 *                ┃  ██║  ██║╚██████╔╝██║  ██║██║  ██║███████╗ ┃              *
 *                ┃  ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝ ┃              *
 *                ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛              *
 *                                                                            *
 *                         Distributed Systems Runtime                        *
 *                                                                            *
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ *
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
 *   limitations under the License.                                           *                                                                             *
 *                                                                            *
\* ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ */

use auraed::*;
use clap::{App, Arg};
use std::path::Path;
use syslog::{BasicLogger, Facility, Formatter3164};

//use futures::Stream;
//use std::{error::Error, io::ErrorKind, net::ToSocketAddrs, path::Path, pin::Pin, time::Duration};
//use tokio::sync::mpsc;
//use tokio_stream::{wrappers::ReceiverStream, StreamExt};
//use tonic::{transport::Server, Request, Response, Status, Streaming};

const EXIT_OKAY: i32 = 0;
//const EXIT_ERROR: i32 = 1;

async fn daemon() -> i32 {
    let name = "auraed";

    // Initialize the program
    let matches = App::new("auraed")
        .version("0.1.0")
        .author("The Aurae Authors")
        .about(name)
        .arg(
            Arg::with_name("verbose")
                .short('v')
                .long("verbose")
                .help("Toggle the verbosity bit.")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("key")
                .short('k')
                .long("key")
                .help("Set a public encryption key: rsa, pem, ed25519, etc")
                .default_value("~/.ssh/id_aurae")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("sock")
                .short('s')
                .long("sock")
                .help("Set a local socket path.")
                .default_value("/var/run/aurae.sock")
                .takes_value(true),
        )
        .get_matches();

    // The logger will log to stdout and the syslog by default.
    // We hold the opinion that the program is either "verbose"
    // or it's not.
    //
    // Normal mode: Info, Warn, Error
    // Verbose mode: Debug, Trace, Info, Warn, Error
    let logger_level = if matches.is_present("verbose") {
        log::Level::Trace
    } else {
        log::Level::Info
    };

    // Syslog formatter
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: name.into(),
        pid: 0,
    };

    // Initialize the logger
    let logger_simple =
        simplelog::SimpleLogger::new(logger_level.to_level_filter(), simplelog::Config::default());
    let logger_syslog = syslog::unix(formatter).unwrap();
    let _ = match multi_log::MultiLogger::init(
        vec![logger_simple, Box::new(BasicLogger::new(logger_syslog))],
        logger_level,
    ) {
        Ok(_) => {}
        Err(e) => panic!("unable to connect to syslog: {:?}", e),
    };

    // Load Variables
    let key = matches.value_of("key").unwrap();
    let sock = matches.value_of("sock").unwrap();

    // Runtime
    runtime(Path::new(sock), Path::new(key));
    return EXIT_OKAY;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exit_code = daemon();
    std::process::exit(exit_code.await);
}
