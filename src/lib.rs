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

mod meta;
mod observe;
mod runtime;

use crate::observe::observe_server::ObserveServer;
use crate::observe::ObserveService;
use crate::runtime::local_runtime_server::LocalRuntimeServer;
use crate::runtime::LocalRuntimeService;

use log::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;
// use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};
use tokio_rustls::{
    rustls::{Certificate, PrivateKey, ServerConfig},
    TlsAcceptor,
};
use tonic::transport::{Identity, Server, ServerTlsConfig};

pub const AURAE_SOCK: &str = "/var/run/aurae/aurae.sock";

#[derive(Debug)]
pub struct AuraedRuntime {
    // Root CA
    pub ca_crt: PathBuf,

    pub server_crt: PathBuf,
    pub server_key: PathBuf,
    pub socket: PathBuf, // TODO replace with namespace
}

impl AuraedRuntime {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Manage the socket permission/groups first\
        let _ = fs::remove_file(&self.socket);
        tokio::fs::create_dir_all(Path::new(&self.socket).parent().unwrap()).await?;
        println!("{:#?}", self);

        let server_crt = tokio::fs::read(&self.server_crt).await?;
        let server_key = tokio::fs::read(&self.server_key).await?;
        let server_identity = Identity::from_pem(server_crt, server_key);
        println!("Register Server SSL Identity");

        // let ca_crt = tokio::fs::read(&self.ca_crt).await?;
        // let ca_crt = Certificate::from_pem(ca_crt);
        // println!("Register Server SSL Certificate Authority (CA)");

        // let new_ca = {
        //     let fd = std::fs::File::open(&self.ca_crt)?;
        //     let mut buf = std::io::BufReader::new(&fd);
        //     rustls_pemfile::certs(&mut buf)?
        //         .into_iter()
        //         .map(Certificate)
        //         .collect()
        // };

        let new_cert = {
            let fd = std::fs::File::open(&self.server_crt)?;
            let mut buf = std::io::BufReader::new(&fd);
            rustls_pemfile::certs(&mut buf)?
                .into_iter()
                .map(Certificate)
                .collect()
        };

        let new_key = {
            let fd = std::fs::File::open(&self.server_key)?;
            let mut buf = std::io::BufReader::new(&fd);
            rustls_pemfile::pkcs8_private_keys(&mut buf)?
                .into_iter()
                .map(PrivateKey)
                .next()
                .unwrap()
        };

        // let tls = ServerTlsConfig::new()
        //     .identity(server_identity)
        //     .client_ca_root(ca_crt);

        let mut tls = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(new_cert, new_key)?;
        println!("Validating SSL Identity and Root Certificate Authority (CA)");

        // Aurae leverages Unix Abstract Sockets
        // Read more about Abstract Sockets: https://man7.org/linux/man-pages/man7/unix.7.html
        // TODO Consider this: https://docs.rs/nix/latest/nix/sys/socket/struct.UnixAddr.html#method.new_abstract
        // let addr = SocketAddr::from_abstract_namespace(b"aurae")?; // Linux only
        // let addr = "[::1]:1234".parse().unwrap();'

        let sock = UnixListener::bind(&self.socket)?;
        // let sock_stream = UnixListenerStream::new(sock);
        let tls_acceptor = TlsAcceptor::from(Arc::new(tls));

        loop {
            let (conn, addr) = match sock.accept().await {
                Ok(res) => res,
                Err(e) => {
                    println!("error: {}", e);
                    continue;
                }
            };

            let tls_acceptor = tls_acceptor.clone();

            let svc = Server::builder()
                .add_service(LocalRuntimeServer::new(LocalRuntimeService::default()))
                .into_service();

            let svc = svc.clone();
            tokio::spawn(async move {

                let mut certificates = Vec::new();
                let conn = tls_acceptor
                    .accept_with(conn, |info| {
                        if let Some(certs) = info.peer_certificates() {
                            for cert in certs {
                                println!("{:x?}", &cert.0);
                                certificates.push(cert.clone());
                            }
                        }
                    })
                    .await
                    .unwrap();

                let svc = tower::ServiceBuilder::new()
                    // .add_extension(Arc::new(ConnInfo { addr, certificates }))
                    .service(svc);

            });
        };

        // // Run the server concurrently
        // // let handle = tokio::spawn(
        // //     Server::builder()
        // //         .add_service(LocalRuntimeServer::new(LocalRuntimeService::default()))
        // //         .add_service(ObserveServer::new(ObserveService::default()))
        // //         .serve_with_incoming(sock_stream),
        // // );
        //
        // println!("Setting socket mode {} -> 766", &self.socket.display());
        //
        // // We set the mode to 766 for the Unix domain socket.
        // // This is what allows non-root users to dial the socket
        // // and authenticate with mTLS.
        // fs::set_permissions(&self.socket, fs::Permissions::from_mode(0o766)).unwrap();
        // println!(
        //     "Non-root User Access Socket Created: {}",
        //     self.socket.display()
        // );
        //
        // // Event loop
        // // let _ = join!(handle);
        // let _ = handle.await?;
        // Ok(())
    }
}
