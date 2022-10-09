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

//use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example running "make command" during the build
    //Command::new("make").args(&["command"]).status().unwrap();

    // gRPC
    tonic_build::configure()
        .server_mod_attribute("meta", "#[allow(clippy::unwrap_used)]")
        .server_mod_attribute("runtime", "#[allow(clippy::unwrap_used)]")
        .server_mod_attribute("observe", "#[allow(clippy::unwrap_used)]")
        .type_attribute(
            "meta.AuraeMeta",
            "#[allow(clippy::derive_partial_eq_without_eq)]",
        )
        .type_attribute(
            "runtime.Executable",
            "#[allow(clippy::derive_partial_eq_without_eq)]",
        )
        .type_attribute(
            "runtime.ExecutableStatus",
            "#[allow(clippy::derive_partial_eq_without_eq)]",
        )
        .compile(
            &[
                "stdlib/v0/meta.proto",
                "stdlib/v0/runtime.proto",
                "stdlib/v0/observe.proto",
            ],
            &["stdlib/v0/"],
        )?;
    Ok(())
}
