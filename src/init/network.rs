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

use anyhow::anyhow;
use futures::stream::TryStreamExt;
use log::{error, info, trace, warn};
use std::collections::HashMap;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str;
use std::{cmp, fs, io};

use ipnetwork::{Ipv4Network, Ipv6Network};

use netlink_packet_route::rtnl::link::nlas::Nla;
use rtnetlink::Handle;

#[derive(Eq, PartialEq)]
enum IpType {
    V6,
    V4,
}

fn get_ip_type(ip: &Vec<u8>) -> Option<IpType> {
    if ip.len() == 4 {
        return Some(IpType::V4);
    } else if ip.len() == 16 {
        return Some(IpType::V6);
    }
    None
}

fn get_sriov_capabilities(iface: &str) -> Result<String, io::Error> {
    fs::read_to_string(format!(
        "/sys/class/net/{}/device/sriov_totalvfs",
        iface
    ))
}

pub(crate) async fn set_link_up(
    handle: Handle,
    iface: &str,
) -> anyhow::Result<()> {
    let mut links = handle.link().get().match_name(iface.to_string()).execute();

    if let Some(link) = links.try_next().await? {
        handle.link().set(link.header.index).up().execute().await?
    } else {
        return Err(anyhow!("iface '{}' not found", iface));
    }
    trace!("Set link {} up", iface);
    Ok(())
}

#[allow(dead_code)]
pub(crate) async fn set_link_down(
    handle: Handle,
    iface: &str,
) -> anyhow::Result<()> {
    let mut links = handle.link().get().match_name(iface.to_string()).execute();

    if let Some(link) = links.try_next().await? {
        handle.link().set(link.header.index).down().execute().await?
    } else {
        return Err(anyhow!("iface '{}' not found", iface));
    }
    trace!("Set link {} down", iface);
    Ok(())
}

pub(crate) async fn add_address_ipv6(
    iface: &str,
    ip: Ipv6Network,
    handle: rtnetlink::Handle,
) -> anyhow::Result<()> {
    let mut links = handle.link().get().match_name(iface.to_string()).execute();

    if let Some(link) = links.try_next().await? {
        handle
            .address()
            .add(link.header.index, std::net::IpAddr::V6(ip.ip()), ip.prefix())
            .execute()
            .await?
    }
    trace!("Added address to link {}", iface);
    Ok(())
}

pub(crate) async fn add_address_ipv4(
    iface: &str,
    ip: Ipv4Network,
    handle: rtnetlink::Handle,
) ->  anyhow::Result<()> {
    let mut links = handle.link().get().match_name(iface.to_string()).execute();

    if let Some(link) = links.try_next().await? {
        handle
            .address()
            .add(link.header.index, std::net::IpAddr::V4(ip.ip()), ip.prefix())
            .execute()
            .await?
    }
    trace!("Added address to link {}", iface);
    Ok(())
}

// Create max(limit, max possible sriov for given iface) sriov devices for the given iface
#[allow(dead_code)]
pub(crate) fn setup_sriov(
    iface: &str,
    limit: u16,
) -> anyhow::Result<()> {
    if limit == 0 {
        return Ok(());
    }

    let sriov_totalvfs = match get_sriov_capabilities(iface) {
        Ok(val) => val,
        Err(e) => {
            return Err(anyhow!("sriov Error: failed to get sriov capabilities of device {}. {}", iface, e));
        }
    };

    let sriov_totalvfs = match sriov_totalvfs.trim_end().parse::<u16>() {
        Ok(val) => val,
        Err(e) => {
            return Err(anyhow!(
                "sriov Error: failed to parse sriov capabilities. {}",
                e
            ));
        }
    };

    let num = cmp::min(limit, sriov_totalvfs);

    fs::write(
        format!("/sys/class/net/{}/device/sriov_numvfs", iface),
        num.to_string(),
    )
    .expect("Unable to write file");
    Ok(())
}

pub(crate) async fn get_links(
    handle: rtnetlink::Handle,
) -> anyhow::Result<HashMap<u32, String>>{
    let mut result = HashMap::new();
    let mut links = handle.link().get().execute();

    'outer: while let Some(msg) = links.try_next().await? {
        for nla in msg.nlas.into_iter() {
            if let Nla::IfName(name) = nla {
                result.insert(msg.header.index, name);
                continue 'outer;
            }
        }
        warn!("link with index {} has no name", msg.header.index);
    }

    Ok(result)
}

pub(crate) fn convert_ipv4_to_string(
    ip: Vec<u8>,
) -> anyhow::Result<String> {
    if ip.len() != 4 {
        return Err(anyhow!("Could not Convert vec: {:?} to ipv4 string", ip));
    }
    let ipv4 = Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3]);
    Ok(ipv4.to_string())
}

pub(crate) fn convert_ipv6_to_string(
    ip: Vec<u8>,
) -> anyhow::Result<String> {
    if ip.len() != 16 {
        return Err(anyhow!("Could not Convert vec: {:?} to ipv6 string", ip));
    }

    let a = ((ip[0] as u16) << 8) | ip[1] as u16;
    let b = ((ip[2] as u16) << 8) | ip[3] as u16;
    let c = ((ip[4] as u16) << 8) | ip[5] as u16;
    let d = ((ip[6] as u16) << 8) | ip[7] as u16;
    let e = ((ip[8] as u16) << 8) | ip[9] as u16;
    let f = ((ip[10] as u16) << 8) | ip[11] as u16;
    let g = ((ip[12] as u16) << 8) | ip[13] as u16;
    let h = ((ip[14] as u16) << 8) | ip[15] as u16;

    let ipv6 = Ipv6Addr::new(a, b, c, d, e, f, g, h);

    Ok(ipv6.to_string())
}

pub(crate) async fn dump_addresses(
    handle: rtnetlink::Handle,
    iface: &str,
) -> anyhow::Result<()> {
    let mut links = handle.link().get().match_name(iface.to_string()).execute();
    if let Some(link_msg) = links.try_next().await? {
        info!("{}:", iface);
        for nla in link_msg.nlas.into_iter() {
            if let Nla::IfName(name) = nla {
                info!("\tindex: {}", link_msg.header.index);
                info!("\tname: {}", name);
            }
        }

        let mut address_msg = handle
            .address()
            .get()
            .set_link_index_filter(link_msg.header.index)
            .execute();

        while let Some(msg) = address_msg.try_next().await? {
            for nla_address in msg.nlas.into_iter() {
                if let netlink_packet_route::address::Nla::Address(addr) =
                    nla_address
                {
                    let ip_type = get_ip_type(&addr);
                    match ip_type {
                        Some(iptype) => {
                            if iptype == IpType::V4 {
                                info!(
                                    "\t ipv4: {}",
                                    convert_ipv4_to_string(addr)
                                        .unwrap_or_else(|_| {
                                            "<error converting ip>".to_string()
                                        })
                                );
                            } else if iptype == IpType::V6 {
                                info!(
                                    "\t ipv6: {}",
                                    convert_ipv6_to_string(addr)
                                        .unwrap_or_else(|_| {
                                            "<error converting ip>".to_string()
                                        })
                                );
                            }
                        }
                        None => {
                            warn!("Failed to get ip type of {:?}", addr);
                        }
                    }
                }
            }
        }
        Ok(())
    } else {
        return Err(anyhow!("link {} not found", iface));
    }
}

pub(crate) async fn show_network_info(handle: rtnetlink::Handle) {
    info!("=== Network Interfaces ===");

    info!("Addresses:");
    let links_result = get_links(handle.clone()).await;

    match links_result {
        Ok(links) => {
            for (_, iface) in links {
                dump_addresses(handle.clone(), &iface).await.unwrap();
            }
        }
        Err(e) => {
            error!("{:?}", e);
        }
    }
    info!("==========================");
}
