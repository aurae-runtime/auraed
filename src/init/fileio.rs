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

use std::fs::{ReadDir, read_dir};

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
