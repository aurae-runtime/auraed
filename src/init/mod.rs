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

pub(crate) mod fileio;
pub(crate) mod network;

use log::{error, info, warn, Level};
use std::ffi::CString;
use std::ptr;
use syslog::{BasicLogger, Facility, Formatter3164};

const AURAED_SYSLOG_NAME: &str = "auraed";

pub fn get_pid() -> u32 {
    std::process::id()
}

pub fn banner() -> String {
    "
    +--------------------------------------------+
    |   █████╗ ██╗   ██╗██████╗  █████╗ ███████╗ |
    |  ██╔══██╗██║   ██║██╔══██╗██╔══██╗██╔════╝ |
    |  ███████║██║   ██║██████╔╝███████║█████╗   |
    |  ██╔══██║██║   ██║██╔══██╗██╔══██║██╔══╝   |
    |  ██║  ██║╚██████╔╝██║  ██║██║  ██║███████╗ |
    |  ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝ |
    +--------------------------------------------+\n"
        .to_string()
}

pub fn print_logo() {
    println!("{}", banner());
}

#[cfg(not(target_os = "macos"))]
fn mount_vfs(source_name: &str, target_name: &str, fstype: &str) {
    info!("Mounting {} as type {}", target_name, fstype);

    // CString constructor ensures the trailing 0byte, which is required by libc::mount
    let src_c_ctr = CString::new(source_name).unwrap();
    let target_name_c_ctr = CString::new(target_name).unwrap();
    let fstype_c_ctr = CString::new(fstype).unwrap();

    let ret = unsafe {
        libc::mount(
            src_c_ctr.as_ptr(),
            target_name_c_ctr.as_ptr(),
            fstype_c_ctr.as_ptr(),
            0,
            ptr::null(),
        )
    };

    if ret < 0 {
        error!("Failed to mount ({})", ret);
        let error = CString::new("Error: ").unwrap();
        unsafe {
            libc::perror(
                error.as_ptr()
            );
        }
    }
}

#[cfg(target_os = "macos")]
fn mount_vfs(source_name: &str, target_name: &str, _fstype: &str) {
    info!("Mounting {}", target_name);

    // CString constructor ensures the trailing 0byte, which is required by libc::mount
    let src_c_ctr = CString::new(source_name).unwrap();
    let target_name_c_ctr = CString::new(target_name).unwrap();

    let ret = unsafe {
        libc::mount(
            src_c_ctr.as_ptr(),
            target_name_c_ctr.as_ptr(),
            0,
            ptr::null_mut(),
        )
    };

    if ret < 0 {
        error!("Failed to mount ({})", ret);
        let error = CString::new("Error: ").unwrap();
        unsafe {
            libc::perror(
                error.as_ptr()
            );
        }
    }
}

pub fn init_rootfs() {
    if get_pid() != 1 {
        warn!("Trying to initialize rootfs but auraed is not run as pid1. Abort setup of rootfs.");
        return;
    }

    mount_vfs("none", "/dev", "devtmpfs");
    mount_vfs("none", "/sys", "sysfs");
    mount_vfs("proc", "/proc", "proc");
}

pub fn init_syslog_logging(logger_level: Level) {
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

    let logger_syslog = match syslog::unix(formatter) {
        Ok(log_val) => log_val,
        Err(e) => {
            panic!("Unable to setup syslog: {:?}", e);
        }
    };

    match multi_log::MultiLogger::init(
        vec![logger_simple, Box::new(BasicLogger::new(logger_syslog))],
        logger_level,
    ) {
        Ok(_) => {}
        Err(e) => panic!("unable to connect to syslog: {:?}", e),
    };
}

// To discuss here https://github.com/aurae-runtime/auraed/issues/24:
//      The "syslog" logger requires unix sockets.
//      Syslog assumes that either /dev/log or /var/run/syslog are available [1].
//      We need to discuss if there is a use case to log via unix sockets,
//      other than fullfill the requirement of syslog crate.
//      For now, auraed distinguishes between pid1 system and local (dev environment) logging.
//      [1] https://docs.rs/syslog/latest/src/syslog/lib.rs.html#232-243
pub fn init_pid1_logging(logger_level: Level) {
    // Initialize the logger
    let logger_simple = simplelog::SimpleLogger::new(
        logger_level.to_level_filter(),
        simplelog::Config::default(),
    );

    match multi_log::MultiLogger::init(vec![logger_simple], logger_level) {
        Ok(_) => {}
        Err(e) => panic!("unable to connect to syslog: {:?}", e),
    };
}
