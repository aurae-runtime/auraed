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

use log::{error, info, trace};
use std::{
    fs::{OpenOptions, File},
    io::Read,
    mem,
    path::Path,
    slice,
};

extern crate libc;

#[allow(dead_code)]
fn syscall_reboot(action: i32) {
    unsafe {
        libc::reboot(action);
    }
}

pub fn power_off() {
    syscall_reboot(libc::LINUX_REBOOT_CMD_POWER_OFF);
}

#[allow(dead_code)]
pub fn reboot() {
    syscall_reboot(libc::LINUX_REBOOT_CMD_RESTART);
}

#[derive(Debug, Default, Copy, Clone)]
#[repr(C, packed)]
pub struct InputEvent {
    tv_sec: u64,
    tv_usec: u64,
    evtype: u16,
    code: u16,
    value: u32,
}

// see  https://elixir.bootlin.com/linux/latest/source/include/uapi/linux/input-event-codes.h#L191
const KEY_POWER: u16 = 116;

#[allow(dead_code)]
pub fn spawn_acpi_listener() {
    // TODO: detect power button devices
    // - multiple power buttons are possible
    // - handle reboot button with a reboot instead
    // - devices are listed in /proc/bus/input/devices
    let power_btn_device = Path::new("/dev/input/event0");

    let mut file_options = OpenOptions::new();
    file_options.read(true);
    file_options.write(false);
    
    let mut event_file: File;
    
    match file_options.open(power_btn_device) {
        Ok(f) => event_file = f,
        Err(e) => {
            error!("Could not open power button device {}. {:?}", power_btn_device.display(), e);
            error!("Stopping acpi power button listener.");
            return;
        }
    }

    let mut event: InputEvent = unsafe { mem::zeroed() };
    let event_size = mem::size_of::<InputEvent>();


    std::thread::spawn(move || loop {
        let event_slice = unsafe {slice::from_raw_parts_mut(&mut event as *mut _ as *mut u8, event_size)};
        match event_file.read(event_slice) {
            Ok(result) => {
                trace!("Event0: {} {:?}",result, event);
                if event.code == KEY_POWER {
                    // TODO: shutdown runtime
                    // - need to send signal via a channel to runtime
                    // - await for runtime
                    info!("Power Button pressed - shutting down now");
                    power_off();
                }
            }
            Err(e) => {
                error!(
                    "Could not parse event from {}: {}",
                    power_btn_device.display(),
                    e
                );
            }
        }

    });

}
