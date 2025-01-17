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

use std::path::PathBuf;

use tokio::fs;

use super::{DirLister, Tenant};
use crate::{async_trait, Error, Result};

pub struct Store {
    path: PathBuf,
}

impl Store {
    pub async fn open(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        fs::create_dir_all(&path).await?;
        Ok(Self { path })
    }
}

#[async_trait]
impl crate::Store for Store {
    fn tenant(&self, name: &str) -> Box<dyn crate::Tenant> {
        Box::new(Tenant::new(self.path.join(name)))
    }

    async fn list_tenants(&self) -> Result<Box<dyn crate::Lister<Item = String>>> {
        let dir = fs::read_dir(&self.path).await?;
        Ok(Box::new(DirLister::new(dir)))
    }

    async fn create_tenant(&self, name: &str) -> Result<Box<dyn crate::Tenant>> {
        let path = self.path.join(name);
        if path.exists() {
            return Err(Error::AlreadyExists(format!("tenant {}", name)));
        }
        fs::create_dir_all(&path).await?;
        Ok(self.tenant(name))
    }

    async fn delete_tenant(&self, name: &str) -> Result<()> {
        let path = self.path.join(name);
        fs::remove_dir_all(&path).await?;
        Ok(())
    }
}
