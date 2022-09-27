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

// Issue tracking: https://github.com/rust-lang/rust/issues/85410
// Here we need to build an abstract socket from a SocketAddr until
// tokio supports abstract sockets natively
// #![feature(unix_socket_abstract)]
// use std::os::unix::net::SocketAddr;

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use init::init_pid1_logging;
use init::init_rootfs;
use init::init_syslog_logging;
use init::print_logo;
use log::*;
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};

use crate::observe::observe_server::ObserveServer;
use crate::observe::ObserveService;
use crate::runtime::runtime_server::RuntimeServer;
use crate::runtime::RuntimeService;

mod codes;
mod init;
mod meta;
mod observe;
mod runtime;

pub const AURAE_SOCK: &str = "/var/run/aurae/aurae.sock";

#[derive(Debug)]
pub struct AuraedRuntime {
    // Root CA
    pub ca_crt: PathBuf,

    pub server_crt: PathBuf,
    pub server_key: PathBuf,
    pub socket: PathBuf,
}

impl AuraedRuntime {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Manage the socket permission/groups first\
        let _ = fs::remove_file(&self.socket);
        tokio::fs::create_dir_all(Path::new(&self.socket).parent().unwrap())
            .await
            .with_context(|| {
                format!(
                    "Failed to create directory for socket: {}",
                    self.socket.display()
                )
            })?;
        trace!("{:#?}", self);

        let server_crt =
            tokio::fs::read(&self.server_crt).await.with_context(|| {
                format!(
                    "Failed to read server certificate: {}",
                    self.server_crt.display()
                )
            })?;
        let server_key = tokio::fs::read(&self.server_key).await?;
        let server_identity = Identity::from_pem(server_crt, server_key);
        info!("Register Server SSL Identity");

        let ca_crt = tokio::fs::read(&self.ca_crt).await?;
        let ca_crt_pem = Certificate::from_pem(ca_crt.clone());

        let tls = ServerTlsConfig::new()
            .identity(server_identity)
            .client_ca_root(ca_crt_pem);

        info!("Validating SSL Identity and Root Certificate Authority (CA)");

        let sock = UnixListener::bind(&self.socket)?;
        let sock_stream = UnixListenerStream::new(sock);

        // Run the server concurrently
        let handle = tokio::spawn(async {
            Server::builder()
                .tls_config(tls)?
                .add_service(RuntimeServer::new(RuntimeService::default()))
                .add_service(ObserveServer::new(ObserveService::default()))
                .serve_with_incoming(sock_stream)
                .await
        });

        trace!("Setting socket mode {} -> 766", &self.socket.display());

        // We set the mode to 766 for the Unix domain socket.
        // This is what allows non-root users to dial the socket
        // and authenticate with mTLS.
        fs::set_permissions(&self.socket, fs::Permissions::from_mode(0o766))
            .unwrap();
        info!("User Access Socket Created: {}", self.socket.display());

        // Event loop
        let res = handle.await.unwrap();

        info!("{:?}", res);
        Ok(())
    }
}

#[derive(Debug)]
pub struct SystemRuntime {
    pub logger_level: Level,
}

impl SystemRuntime {
    fn init_pid1(&self) {
        print_logo();

        init_pid1_logging(self.logger_level);

        init_rootfs();

        // Show content of file-based kernel interface directories
        //fileio::show_dir("/dev", false);
        //fileio::show_dir("/sys", false);
        //fileio::show_dir("/proc", false);

        // Implement & Discuss (TODO): init::init_network();
        // Show available network interfaces
        // fileio::show_dir("/sys/class/net/", false);
    }

    fn init_pid_gt_1(&self) {
        init_syslog_logging(self.logger_level);
    }

    pub fn init(&self) {
        if init::get_pid() == 1 {
            self.init_pid1();
        } else {
            self.init_pid_gt_1();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_path() {
        assert_eq!(AURAE_SOCK, "/var/run/aurae/aurae.sock");
    }
}
