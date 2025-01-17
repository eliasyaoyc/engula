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

#![allow(clippy::all)]

pub use engula_apis::v1::{
    universe_request, universe_response, CollectionDesc, CollectionOptions, CollectionProperties,
    CreateCollectionRequest, CreateCollectionResponse, CreateDatabaseRequest,
    CreateDatabaseResponse, DatabaseDesc, DatabaseOptions, DatabaseProperties,
    DeleteCollectionRequest, DeleteCollectionResponse, DeleteDatabaseRequest,
    DeleteDatabaseResponse, DescribeCollectionRequest, DescribeCollectionResponse,
    DescribeDatabaseRequest, DescribeDatabaseResponse, ListCollectionsRequest,
    ListCollectionsResponse, ListDatabasesRequest, ListDatabasesResponse, UniverseRequest,
    UniverseResponse,
};

tonic::include_proto!("engula.supervisor.v1");
