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

use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use tokio::sync::Mutex;

use crate::{apis::*, Error, Result};

#[derive(Clone)]
pub struct Universe {
    inner: Arc<Mutex<UniverseInner>>,
}

impl Default for Universe {
    fn default() -> Self {
        Self::new()
    }
}

impl Universe {
    pub fn new() -> Self {
        let inner = UniverseInner::new();
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    pub async fn database(&self, name: &str) -> Result<Database> {
        let inner = self.inner.lock().await;
        inner.database(name)
    }

    pub async fn list_databases(
        &self,
        count: usize,
        token: Option<String>,
    ) -> Result<(Vec<Database>, Option<String>)> {
        let inner = self.inner.lock().await;
        inner.list_databases(count, token).await
    }

    pub async fn create_database(&self, name: &str, options: DatabaseOptions) -> Result<Database> {
        let mut inner = self.inner.lock().await;
        inner.create_database(name, options)
    }

    pub async fn delete_database(&self, name: &str) -> Result<()> {
        let mut inner = self.inner.lock().await;
        inner.delete_database(name)
    }
}

struct UniverseInner {
    next_id: AtomicU64,
    databases: BTreeMap<String, Database>,
}

impl UniverseInner {
    fn new() -> Self {
        Self {
            next_id: AtomicU64::new(1),
            databases: BTreeMap::new(),
        }
    }

    fn database(&self, name: &str) -> Result<Database> {
        self.databases
            .get(name)
            .cloned()
            .ok_or_else(|| Error::NotFound(format!("database {}", name)))
    }

    async fn list_databases(
        &self,
        count: usize,
        token: Option<String>,
    ) -> Result<(Vec<Database>, Option<String>)> {
        let token = token.unwrap_or_default();
        let dbs = self
            .databases
            .range(token.clone()..)
            .take(count)
            .map(|(_, db)| db.clone())
            .collect();
        let next_token = self
            .databases
            .range(token..)
            .nth(count)
            .map(|(name, _)| name.to_owned());
        Ok((dbs, next_token))
    }

    fn create_database(&mut self, name: &str, options: DatabaseOptions) -> Result<Database> {
        if self.databases.contains_key(name) {
            return Err(Error::AlreadyExists(format!("database {}", name)));
        }
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let db = Database::new(id, name.to_owned(), options);
        self.databases.insert(name.to_owned(), db.clone());
        Ok(db)
    }

    fn delete_database(&mut self, name: &str) -> Result<()> {
        self.databases.remove(name);
        Ok(())
    }
}

#[derive(Clone)]
pub struct Database {
    inner: Arc<Mutex<DatabaseInner>>,
}

impl Database {
    fn new(id: u64, name: String, options: DatabaseOptions) -> Self {
        let inner = DatabaseInner::new(id, name, options);
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    pub async fn desc(&self) -> DatabaseDesc {
        self.inner.lock().await.desc()
    }

    pub async fn collection(&self, name: &str) -> Result<Collection> {
        let inner = self.inner.lock().await;
        inner.collection(name)
    }

    pub async fn list_collections(
        &self,
        count: usize,
        token: Option<String>,
    ) -> Result<(Vec<Collection>, Option<String>)> {
        let inner = self.inner.lock().await;
        inner.list_collections(count, token).await
    }

    pub async fn create_collection(
        &self,
        name: &str,
        options: CollectionOptions,
    ) -> Result<Collection> {
        let mut inner = self.inner.lock().await;
        inner.create_collection(name, options)
    }

    pub async fn delete_collection(&self, name: &str) -> Result<()> {
        let mut inner = self.inner.lock().await;
        inner.delete_collection(name)
    }
}

struct DatabaseInner {
    id: u64,
    name: String,
    options: DatabaseOptions,
    next_id: AtomicU64,
    collections: BTreeMap<String, Collection>,
}

impl DatabaseInner {
    fn new(id: u64, name: String, options: DatabaseOptions) -> Self {
        Self {
            id,
            name,
            options,
            next_id: AtomicU64::new(1),
            collections: BTreeMap::new(),
        }
    }

    fn desc(&self) -> DatabaseDesc {
        let properties = DatabaseProperties {
            num_collections: self.collections.len() as u64,
        };
        DatabaseDesc {
            id: self.id,
            name: self.name.clone(),
            options: Some(self.options.clone()),
            properties: Some(properties),
        }
    }

    fn collection(&self, name: &str) -> Result<Collection> {
        self.collections
            .get(name)
            .cloned()
            .ok_or_else(|| Error::NotFound(format!("collection {}", name)))
    }

    pub async fn list_collections(
        &self,
        count: usize,
        token: Option<String>,
    ) -> Result<(Vec<Collection>, Option<String>)> {
        let token = token.unwrap_or_default();
        let cos = self
            .collections
            .range(token.clone()..)
            .take(count)
            .map(|(_, co)| co.clone())
            .collect();
        let next_token = self
            .collections
            .range(token..)
            .nth(count)
            .map(|(name, _)| name.to_owned());
        Ok((cos, next_token))
    }

    fn create_collection(&mut self, name: &str, options: CollectionOptions) -> Result<Collection> {
        if self.collections.contains_key(name) {
            return Err(Error::AlreadyExists(format!("collection {}", name)));
        }
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let co = Collection::new(id, name.to_owned(), options);
        self.collections.insert(name.to_owned(), co.clone());
        Ok(co)
    }

    fn delete_collection(&mut self, name: &str) -> Result<()> {
        self.collections.remove(name);
        Ok(())
    }
}

#[derive(Clone)]
pub struct Collection {
    inner: Arc<Mutex<CollectionInner>>,
}

impl Collection {
    fn new(id: u64, name: String, options: CollectionOptions) -> Self {
        let inner = CollectionInner::new(id, name, options);
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    pub async fn desc(&self) -> CollectionDesc {
        self.inner.lock().await.desc()
    }
}

struct CollectionInner {
    id: u64,
    name: String,
    options: CollectionOptions,
}

impl CollectionInner {
    fn new(id: u64, name: String, options: CollectionOptions) -> Self {
        Self { id, name, options }
    }

    fn desc(&self) -> CollectionDesc {
        CollectionDesc {
            id: self.id,
            name: self.name.clone(),
            options: Some(self.options.clone()),
            ..Default::default()
        }
    }
}
