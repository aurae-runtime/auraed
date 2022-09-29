use log::*;
use std::ffi::CString;
use syslog::{BasicLogger, Facility, Formatter3164};

use super::*;

fn mount_vfs(source_name: &str, target_name: &str, fstype: &str) {
    info!("Mounting {} as type {}", target_name, fstype);
    unsafe {
        // CString constructor ensures the trailing 0byte, which is required by libc::mount
        let src_c_ctr = CString::new(source_name).unwrap();
        let target_name_c_ctr = CString::new(target_name).unwrap();
        let fstype_c_ctr = CString::new(fstype).unwrap();
        let options_c_ctr = CString::new("").unwrap();

        let ret = libc::mount(
            src_c_ctr.as_ptr() as *const i8,
            target_name_c_ctr.as_ptr() as *const i8,
            fstype_c_ctr.as_ptr() as *const i8,
            0,
            options_c_ctr.as_ptr() as *const libc::c_void,
        );

        if ret < 0 {
            error!("Failed to mount ({})", ret);
            libc::perror(
                String::from("Error: ").as_bytes().as_ptr() as *const i8
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

pub fn init_local_logging(logger_level: Level) {
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

// To Discuss (TODO):
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
