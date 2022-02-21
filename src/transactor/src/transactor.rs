// Copyright 2022 The Engula Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use engula_apis::*;
use engula_cooperator::Cooperator;
use engula_supervisor::Server as Supervisor;
use tonic::{Request, Response, Status};

type TonicResult<T> = std::result::Result<T, Status>;

pub struct Transactor {
    supervisor: Supervisor,
    cooperator: Cooperator,
}

impl Default for Transactor {
    fn default() -> Self {
        Self::new()
    }
}

impl Transactor {
    pub fn new() -> Self {
        Self {
            supervisor: Supervisor::new(),
            cooperator: Cooperator::new(),
        }
    }

    pub fn into_service(self) -> engula_server::EngulaServer<Self> {
        engula_server::EngulaServer::new(self)
    }
}

#[tonic::async_trait]
impl engula_server::Engula for Transactor {
    async fn txn(&self, req: Request<TxnRequest>) -> TonicResult<Response<TxnResponse>> {
        let req = req.into_inner();
        let res = self.cooperator.execute(req).await?;
        Ok(Response::new(res))
    }

    async fn database(
        &self,
        req: Request<DatabaseRequest>,
    ) -> TonicResult<Response<DatabaseResponse>> {
        self.supervisor.database(req).await
    }

    async fn collection(
        &self,
        req: Request<CollectionRequest>,
    ) -> TonicResult<Response<CollectionResponse>> {
        self.supervisor.collection(req).await
    }
}
