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

use futures::future::join_all;

use crate::{apis::*, Error, Result, Universe};

#[derive(Clone, Default)]
pub struct Supervisor {
    uv: Universe,
}

impl Supervisor {
    pub async fn batch(&self, batch_req: BatchRequest) -> Result<BatchResponse> {
        let mut batch_res = BatchResponse::default();
        for req in batch_req.universes {
            let res = self.universe(req).await?;
            batch_res.universes.push(res);
        }
        Ok(batch_res)
    }

    async fn universe(&self, req: UniverseRequest) -> Result<UniverseResponse> {
        let req = req
            .request
            .ok_or_else(|| Error::invalid_argument("missing request"))?;
        let res = match req {
            universe_request::Request::ListDatabases(req) => {
                let res = self.list_databases(req).await?;
                universe_response::Response::ListDatabases(res)
            }
            universe_request::Request::CreateDatabase(req) => {
                let res = self.create_database(req).await?;
                universe_response::Response::CreateDatabase(res)
            }
            universe_request::Request::DeleteDatabase(req) => {
                let res = self.delete_database(req).await?;
                universe_response::Response::DeleteDatabase(res)
            }
            universe_request::Request::DescribeDatabase(req) => {
                let res = self.describe_database(req).await?;
                universe_response::Response::DescribeDatabase(res)
            }
            universe_request::Request::ListCollections(req) => {
                let res = self.list_collections(req).await?;
                universe_response::Response::ListCollections(res)
            }
            universe_request::Request::CreateCollection(req) => {
                let res = self.create_collection(req).await?;
                universe_response::Response::CreateCollection(res)
            }
            universe_request::Request::DeleteCollection(req) => {
                let res = self.delete_collection(req).await?;
                universe_response::Response::DeleteCollection(res)
            }
            universe_request::Request::DescribeCollection(req) => {
                let res = self.describe_collection(req).await?;
                universe_response::Response::DescribeCollection(res)
            }
            _ => {
                todo!();
            }
        };
        Ok(UniverseResponse {
            response: Some(res),
        })
    }

    pub async fn list_databases(&self, req: ListDatabasesRequest) -> Result<ListDatabasesResponse> {
        let (dbs, next_token) = self
            .uv
            .list_databases(req.page_size as usize, Some(req.page_token))
            .await?;
        let descs = join_all(dbs.iter().map(|db| db.desc())).await;
        Ok(ListDatabasesResponse {
            descs,
            next_page_token: next_token.unwrap_or_default(),
        })
    }

    pub async fn create_database(
        &self,
        req: CreateDatabaseRequest,
    ) -> Result<CreateDatabaseResponse> {
        let db = self
            .uv
            .create_database(&req.name, req.options.unwrap_or_default())
            .await?;
        let desc = db.desc().await;
        Ok(CreateDatabaseResponse { desc: Some(desc) })
    }

    pub async fn delete_database(
        &self,
        req: DeleteDatabaseRequest,
    ) -> Result<DeleteDatabaseResponse> {
        self.uv.delete_database(&req.name).await?;
        Ok(DeleteDatabaseResponse {})
    }

    pub async fn describe_database(
        &self,
        req: DescribeDatabaseRequest,
    ) -> Result<DescribeDatabaseResponse> {
        let db = self.uv.database(&req.name).await?;
        let desc = db.desc().await;
        Ok(DescribeDatabaseResponse { desc: Some(desc) })
    }

    pub async fn list_collections(
        &self,
        req: ListCollectionsRequest,
    ) -> Result<ListCollectionsResponse> {
        let db = self.uv.database(&req.name).await?;
        let (cos, next_token) = db
            .list_collections(req.page_size as usize, Some(req.page_token))
            .await?;
        let descs = join_all(cos.iter().map(|co| co.desc())).await;
        Ok(ListCollectionsResponse {
            descs,
            next_page_token: next_token.unwrap_or_default(),
        })
    }

    pub async fn create_collection(
        &self,
        req: CreateCollectionRequest,
    ) -> Result<CreateCollectionResponse> {
        let db = self.uv.database(&req.dbname).await?;
        let co = db
            .create_collection(&req.name, req.options.unwrap_or_default())
            .await?;
        let desc = co.desc().await;
        Ok(CreateCollectionResponse { desc: Some(desc) })
    }

    pub async fn delete_collection(
        &self,
        req: DeleteCollectionRequest,
    ) -> Result<DeleteCollectionResponse> {
        let db = self.uv.database(&req.dbname).await?;
        db.delete_collection(&req.name).await?;
        Ok(DeleteCollectionResponse {})
    }

    pub async fn describe_collection(
        &self,
        req: DescribeCollectionRequest,
    ) -> Result<DescribeCollectionResponse> {
        let db = self.uv.database(&req.dbname).await?;
        let co = db.collection(&req.name).await?;
        let desc = co.desc().await;
        Ok(DescribeCollectionResponse { desc: Some(desc) })
    }
}
