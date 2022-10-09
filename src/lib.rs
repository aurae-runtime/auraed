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
#![allow(clippy::derive_partial_eq_without_eq)]

use anyhow::anyhow;
use anyhow::Context;
use init::init_pid1_logging;
use init::init_rootfs;
use init::init_syslog_logging;
use init::network::show_network_info;
use init::print_logo;
use log::*;
use netlink_packet_route::RtnlMessage;
use rtnetlink::new_connection;
use rtnetlink::proto::Connection;
use sea_orm::ConnectOptions;
use sea_orm::ConnectionTrait;
use sea_orm::Database;
use sea_orm::Statement;
use std::borrow::Cow;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};

use crate::init::network::set_link_up;
use crate::init::network::{add_address, add_route_v6};
use crate::init::power::spawn_thread_power_button_listener;

use ipnetwork::{IpNetwork, Ipv6Network};

// use crate::init::fileio::show_dir;
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
pub const LOOPBACK_DEV: &str = "lo";

pub const LOOPBACK_IPV6: &str = "::1";
pub const LOOPBACK_IPV6_SUBNET: &str = "/128";

pub const LOOPBACK_IPV4: &str = "127.0.0.1";
pub const LOOPBACK_IPV4_SUBNET: &str = "/8";

pub const DEFAULT_NET_DEV: &str = "eth0";
pub const DEFAULT_NET_DEV_IPV6: &str = "fe80::2";
pub const DEFAULT_NET_DEV_IPV6_SUBNET: &str = "/64";

pub const POWER_BUTTON_DEVICE: &str = "/dev/input/event0";

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
        let sock_res = Path::new(&self.socket)
            .parent()
            .ok_or("unable to find socket path");
        let sock_path = match sock_res {
            Ok(path) => path,
            Err(e) => return Err(e.into()),
        };
        tokio::fs::create_dir_all(sock_path).await.with_context(|| {
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
        let db_key = server_key.clone();
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
        fs::set_permissions(&self.socket, fs::Permissions::from_mode(0o766))?;
        info!("User Access Socket Created: {}", self.socket.display());

        // SQLite
        info!("Database Location:  /var/lib/aurae.db");
        info!("Unlocking SQLite Database with Key: {:?}", self.server_key);
        let mut opt =
            ConnectOptions::new("sqlite:/var/lib/aurae.db".to_owned());
        opt.sqlx_logging(false).sqlcipher_key(Cow::from(format!(
            "{:?}",
            db_key.to_ascii_lowercase()
        )));

        // Pragma initial connection
        let mut opt = ConnectOptions::new("sqlite::memory:".to_owned());
        opt.sqlx_logging(false); // TODO add sqlcipher_key
        let db = Database::connect(opt).await?;
        let x = db
            .execute(Statement::from_string(
                db.get_database_backend(),
                "PRAGMA database_list;".to_string(),
            ))
            .await?;
        info!("Initializing: SQLite: {:?}", x);

        //runtime::hydrate(&db).await?;

        // Event loop
        let res = handle.await?;
        match res {
            Ok(_) => {
                info!("{:?}", res);
            }
            Err(e) => return Err(e.into()),
        };

        Ok(())
    }
}

#[derive(Debug)]
pub struct SystemRuntime {
    pub logger_level: Level,
}

impl SystemRuntime {
    fn spawn_system_runtime_threads(&self) {
        // ---- MAIN DAEMON THREAD POOL ----
        // TODO: https://github.com/aurae-runtime/auraed/issues/33
        match spawn_thread_power_button_listener(Path::new(POWER_BUTTON_DEVICE))
        {
            Ok(_) => {
                info!("Spawned power button device listener");
            }
            Err(e) => {
                error!(
                    "Failed to spawn power button device listener. Error={}",
                    e
                );
            }
        }

        // ---- MAIN DAEMON THREAD POOL ----
    }

    async fn configure_loopback(
        &self,
        handle: &rtnetlink::Handle,
    ) -> anyhow::Result<()> {
        if let Ok(ipv6) = format!("{}{}", LOOPBACK_IPV6, LOOPBACK_IPV6_SUBNET)
            .parse::<IpNetwork>()
        {
            if let Err(e) = add_address(LOOPBACK_DEV, ipv6, handle).await {
                return Err(anyhow!("Failed to add ipv6 address to loopback device {}. Error={}", LOOPBACK_DEV, e));
            };
        }

        if let Ok(ipv4) = format!("{}{}", LOOPBACK_IPV4, LOOPBACK_IPV4_SUBNET)
            .parse::<IpNetwork>()
        {
            if let Err(e) = add_address(LOOPBACK_DEV, ipv4, handle).await {
                return Err(anyhow!("Failed to add ipv4 address to loopback device {}. Error={}", LOOPBACK_DEV, e));
            }
        };

        if let Err(e) = set_link_up(handle, LOOPBACK_DEV).await {
            return Err(anyhow!(
                "Failed to set link up for device {}. Error={}",
                LOOPBACK_DEV,
                e
            ));
        }

        Ok(())
    }

    // TODO: design network config struct
    async fn configure_nic(
        &self,
        handle: &rtnetlink::Handle,
    ) -> anyhow::Result<()> {
        if let Ok(ipv6) =
            format!("{}{}", DEFAULT_NET_DEV_IPV6, DEFAULT_NET_DEV_IPV6_SUBNET)
                .parse::<Ipv6Network>()
        {
            if let Err(e) = add_address(DEFAULT_NET_DEV, ipv6, handle).await {
                return Err(anyhow!(
                    "Failed to add ipv6 address to device {}. Error={}",
                    DEFAULT_NET_DEV,
                    e
                ));
            }

            if let Err(e) = set_link_up(handle, DEFAULT_NET_DEV).await {
                return Err(anyhow!(
                    "Failed to set link up for device {}. Error={}",
                    DEFAULT_NET_DEV,
                    e
                ));
            }

            if let Ok(destv6) = "::/0".to_string().parse::<Ipv6Network>() {
                if let Err(e) =
                    add_route_v6(&destv6, DEFAULT_NET_DEV, &ipv6, handle).await
                {
                    return Err(anyhow!(
                        "Failed to add ipv6 route to device {}. Error={}",
                        DEFAULT_NET_DEV,
                        e
                    ));
                }
            }
        };

        Ok(())
    }

    async fn init_pid1_network(
        &self,
        connection: Connection<RtnlMessage>,
        handle: &rtnetlink::Handle,
    ) {
        tokio::spawn(connection);

        trace!("configure {}", LOOPBACK_DEV);
        match self.configure_loopback(handle).await {
            Ok(_) => {
                info!("Successfully configured {}", LOOPBACK_DEV);
            }
            Err(e) => {
                error!("Failed to setup loopback device. Error={}", e);
            }
        }

        trace!("configure {}", DEFAULT_NET_DEV);

        match self.configure_nic(handle).await {
            Ok(_) => {
                info!("Successfully configured {}", DEFAULT_NET_DEV);
            }
            Err(e) => {
                error!(
                    "Failed to configure NIC {}. Error={}",
                    DEFAULT_NET_DEV, e
                );
            }
        }

        show_network_info(handle).await;
    }

    async fn init_pid1(&self) {
        print_logo();

        init_pid1_logging(self.logger_level);
        trace!("Logging started");

        trace!("Configure filesystem");
        init_rootfs();

        trace!("configure network");
        //show_dir("/sys/class/net/", false); // Show available network interfaces
        match new_connection() {
            Ok((connection, handle, ..)) => {
                self.init_pid1_network(connection, &handle).await;
            }
            Err(e) => {
                error!("Could not initialize network! Error={}", e);
            }
        };

        self.spawn_system_runtime_threads();

        trace!("init of auraed as pid1 done");
    }

    fn init_pid_gt_1(&self) {
        init_syslog_logging(self.logger_level);
    }

    pub async fn init(&self) {
        if init::get_pid() == 1 {
            self.init_pid1().await;
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
