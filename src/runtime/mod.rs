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

tonic::include_proto!("runtime");
tonic::include_proto!("meta");

use crate::codes::*;
use crate::meta;
use crate::runtime::runtime_server::Runtime;
use tonic::{Request, Response, Status};

#[derive(Debug, Default, Clone)]
pub struct RuntimeService {}

#[tonic::async_trait]
impl Runtime for RuntimeService {
    async fn start_executable(
        &self,
        req: Request<Executable>,
    ) -> Result<Response<ExecutableStatus>, Status> {
        let mut meta = Vec::new();
        meta.push(meta::AuraeMeta {
            code: CODE_SUCCESS,
            message: STATUS_READY.into(),
        });
        let response = ExecutableStatus {
            meta,
            state: STATE_ACTIVE.into(),
            name: req.into_inner().name,
        };
        Ok(Response::new(response))
    }
    async fn stop_executable(
        &self,
        req: Request<Executable>,
    ) -> Result<Response<ExecutableStatus>, Status> {
        let mut meta = Vec::new();
        meta.push(meta::AuraeMeta {
            code: CODE_SUCCESS,
            message: STATUS_READY.into(),
        });
        let response = ExecutableStatus {
            meta,
            state: STATE_ACTIVE.into(),
            name: req.into_inner().name,
        };
        Ok(Response::new(response))
    }
    async fn register_executable(
        &self,
        req: Request<Executable>,
    ) -> Result<Response<ExecutableStatus>, Status> {
        let mut meta = Vec::new();
        meta.push(meta::AuraeMeta {
            code: CODE_SUCCESS,
            message: STATUS_READY.into(),
        });
        let response = ExecutableStatus {
            meta,
            state: STATE_ACTIVE.into(),
            name: req.into_inner().name,
        };
        Ok(Response::new(response))
    }
    async fn destroy_executable(
        &self,
        req: Request<Executable>,
    ) -> Result<Response<ExecutableStatus>, Status> {
        let mut meta = Vec::new();
        meta.push(meta::AuraeMeta {
            code: CODE_SUCCESS,
            message: STATUS_READY.into(),
        });
        let response = ExecutableStatus {
            meta,
            state: STATE_ACTIVE.into(),
            name: req.into_inner().name,
        };
        Ok(Response::new(response))
    }
}
