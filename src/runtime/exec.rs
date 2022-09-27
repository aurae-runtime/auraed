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
use std::process::Child;

// Spawn is a child process of Auraed.
//
// Spawn:
// Executes the command as a child process, returning a handle to it.
// By default, stdin, stdout and stderr are inherited from the parent.
pub struct Spawn {
    pub process: Child,
}

use std::process::Command;

pub fn exec(cmd: &str) -> Result<Spawn, anyhow::Error> {
    let spl = cmd.split(" ");
    let ents: Vec<&str> = spl.collect();
    if ents.len() < 1 {
        return Err(anyhow!("empty argument command string"));
    }

    // Build the base command ents[0]
    let mut x = Command::new(ents[0].clone());
    let c = ents[0].clone();

    // Add arguments if they exist
    if ents.len() > 1 {
        for ent in ents {
            if ent == c {
                continue;
            }
            x.arg(ent);
        }
    }

    // Spawn
    // Executes the command as a child process, returning a handle to it.
    // By default, stdin, stdout and stderr are inherited from the parent.
    let child = x.spawn()?;
    let spawn = Spawn { process: child };
    Ok(spawn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commands_exec() {
        // test_exec will test "concurrent" processes
        // each of these spawn and return before exiting
        let spawn1 = exec("touch /tmp/.aurae.testfile");
        assert!(spawn1.is_ok());

        let spawn2 = exec("cat /tmp/.aurae.testfile");
        assert!(spawn2.is_ok());

        let spawn3 = exec("this-is-a-known-bad-command-executable");
        assert!(spawn3.is_err());

        let spawn4 = exec(""); // empty arguments should fail
        assert!(spawn4.is_err());
    }
}
