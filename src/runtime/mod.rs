/* -------------------------------------------------------------------------- *\
 *             Apache 2.0 License Copyright © 2022 The Aurae Authors          *
 *                                                                            *
 *                ┏--------------------------------------------┓              *
 *                ┃   █████╗ ██╗   ██╗██████╗  █████╗ ███████╗ ┃              *
 *                ┃  ██╔══██╗██║   ██║██╔══██╗██╔══██╗██╔════╝ ┃              *
 *                ┃  ███████║██║   ██║██████╔╝███████║█████╗   ┃              *
 *                ┃  ██╔══██║██║   ██║██╔══██╗██╔══██║██╔══╝   ┃              *
 *                ┃  ██║  ██║╚██████╔╝██║  ██║██║  ██║███████╗ ┃              *
 *                ┃  ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝ ┃              *
 *                ┗--------------------------------------------┛              *
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
 *   limitations under the License.                                           *                                                                             *
 *                                                                            *
\* -------------------------------------------------------------------------- */

tonic::include_proto!("runtime");

use crate::runtime::local_runtime_server::LocalRuntime;
use tonic::{Request, Response, Status};

#[derive(Debug, Default, Clone)]
pub struct LocalRuntimeService {}

#[tonic::async_trait]
impl LocalRuntime for LocalRuntimeService {
    /// RunProcess is modelled off systemd "ExecStart" which calls fork(2)
    async fn run_process(
        &self,
        _request: Request<RunProcessRequest>,
    ) -> Result<Response<RunProcessResponse>, Status> {
        todo!()
    }
    async fn read_stdout(
        &self,
        _request: Request<ReadStdoutRequest>,
    ) -> Result<Response<ReadStdoutResponse>, Status> {
        todo!()
    }
    async fn read_stderr(
        &self,
        _request: Request<ReadStderrRequest>,
    ) -> Result<Response<ReadStderrResponse>, Status> {
        todo!()
    }
}
