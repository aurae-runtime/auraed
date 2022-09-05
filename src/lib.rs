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

use crate::runtime::local_runtime_server::LocalRuntimeServer;
use crate::runtime::LocalRuntimeService;

use log::*;
use std::path::PathBuf;
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;
mod meta;
mod runtime;

#[derive(Debug)]
pub struct AuraedRuntime {
    pub server_crt: PathBuf,
    pub server_key: PathBuf,
    pub ca_crt: PathBuf,
    pub socket: PathBuf,
}

impl AuraedRuntime {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        trace!("{:#?}", self);
        let sock = UnixListener::bind(&self.socket)?;
        let sock_stream = UnixListenerStream::new(sock);
        info!("Starting Socket: {}", self.socket.display());
        let runtime_service = LocalRuntimeService::default();
        Server::builder()
            .add_service(LocalRuntimeServer::new(runtime_service))
            .serve_with_incoming(sock_stream)
            .await?;
        Ok(())
    }
}
