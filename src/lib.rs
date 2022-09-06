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

mod meta;
mod observe;
mod runtime;

use crate::observe::observe_server::ObserveServer;
use crate::observe::ObserveService;
use crate::runtime::local_runtime_server::LocalRuntimeServer;
use crate::runtime::LocalRuntimeService;

use log::*;
use std::fs;
use std::path::PathBuf;
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};

#[derive(Debug)]
pub struct AuraedRuntime {
    pub server_crt: PathBuf,
    pub server_key: PathBuf,
    pub ca_crt: PathBuf,
    pub socket: PathBuf,
}

impl AuraedRuntime {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = fs::remove_file(&self.socket);
        trace!("{:#?}", self);

        let server_crt = tokio::fs::read(&self.server_crt).await?;
        let server_key = tokio::fs::read(&self.server_key).await?;
        let server_identity = Identity::from_pem(server_crt, server_key);
        info!("Register Server SSL Identity");

        let ca_crt = tokio::fs::read(&self.ca_crt).await?;
        let ca_crt = Certificate::from_pem(ca_crt);
        info!("Register Server SSL Certificate Authority (CA)");

        let tls = ServerTlsConfig::new()
            .identity(server_identity)
            .client_ca_root(ca_crt);
        info!("Validating SSL Identity and Root Certificate Authority (CA)");

        let sock = UnixListener::bind(&self.socket)?;
        let sock_stream = UnixListenerStream::new(sock);
        info!("Starting Socket: {}", self.socket.display());
        Server::builder()
            .tls_config(tls)?
            .add_service(LocalRuntimeServer::new(LocalRuntimeService::default()))
            .add_service(ObserveServer::new(ObserveService::default()))
            .serve_with_incoming(sock_stream)
            .await?;
        Ok(())
    }
}
