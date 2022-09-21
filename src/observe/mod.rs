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

tonic::include_proto!("observe");
tonic::include_proto!("meta");

use crate::meta; //  For AuraeMeta type
use crate::meta::*;
use crate::observe::observe_server::Observe;
use tonic::{Request, Response, Status};

#[allow(dead_code)]
pub const STATUS_UNKNOWN: &str = "Unknown";
#[allow(dead_code)]
pub const STATUS_READY: &str = "Ready";
#[allow(dead_code)]
pub const STATUS_ERROR: &str = "Error";

#[derive(Debug, Default, Clone)]
pub struct ObserveService {}

#[tonic::async_trait]
impl Observe for ObserveService {
    async fn status(
        &self,
        _request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        let mut meta = Vec::new();
        meta.push(meta::AuraeMeta {
            code: CODE_UNKNOWN,
            message: MESSAGE_UNKNOWN.into(),
        });
        let response = StatusResponse { meta, state: STATUS_UNKNOWN.into() };
        Ok(Response::new(response))
    }
}
