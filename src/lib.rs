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

use log::*;
use std::path::PathBuf;

#[derive(Debug)]
pub struct AuraedRuntime {
    pub server_crt: PathBuf,
    pub server_key: PathBuf,
    pub ca_crt: PathBuf,
    pub socket: PathBuf,
}

impl AuraedRuntime {
    pub fn run(&self) {
        trace!("{:#?}", self);
        info!("Runtime Started!")

        // let server = EchoServer {};
        // Server::builder()
        //
        //     .add_service(pb::echo_server::EchoServer::new(server))
        //     .serve("[::1]:50051".to_socket_addrs().unwrap().next().unwrap())
        //     .await
        //     .unwrap();
    }
}
