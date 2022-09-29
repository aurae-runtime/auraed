<<<<<<< HEAD
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

use std::fs::{read_dir, ReadDir};
=======
use std::fs::{ReadDir, read_dir};
>>>>>>> 24989c8 (rename system to init)

use walkdir::WalkDir;

fn print_flat_dir(paths: ReadDir) {
    for path in paths {
        match path {
            Err(p) => println!("Error: {}", p),
            Ok(p) => println!("{}", p.path().display()),
        }
    }
}

#[allow(dead_code)]
pub fn show_dir(dir: &str, recurse: bool) {
    if recurse {
        for entry in WalkDir::new(dir) {
            let entry = entry;
            match entry {
                Ok(p) => println!("{}", p.path().display()),
                Err(e) => println!("Could not read {}. Error: {}", dir, e),
            }
        }
    } else {
        let paths = read_dir(dir);

        match paths {
            Ok(paths) => print_flat_dir(paths),
            Err(e) => println!("Could not read {}. Error: {}", dir, e),
        }
    }
}
<<<<<<< HEAD
=======
<<<<<<<< HEAD:src/system/fileio.rs

#[allow(dead_code)]
fn write_file(file_path: &str, content: &str) {
    unsafe {
        let fp: *mut libc::FILE = libc::fopen(
            String::from(file_path).as_bytes().as_ptr() as *const i8,
            String::from("w").as_bytes().as_ptr() as *const i8,
        );

        libc::fprintf(
            fp,
            String::from(content).as_bytes().as_ptr() as *const i8,
        );
        libc::fclose(fp);
    }
}
========
>>>>>>>> 24989c8 (rename system to init):src/init/fileio.rs
>>>>>>> 24989c8 (rename system to init)
