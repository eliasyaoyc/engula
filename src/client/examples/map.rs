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

use std::collections::HashMap;

use anyhow::Result;
use engula_client::{Map, Universe};

#[tokio::main]
async fn main() -> Result<()> {
    let url = "http://localhost:21716";
    let uv = Universe::connect(url).await?;
    let db = uv.create_database("map").await?;
    let co = db.create_collection("map").await?;

    let va = [(0, 0), (1, 1), (2, 2)];
    let vb = [(3, 3), (4, 4), (5, 5)];

    co.set("a", Map::value(va)).await?;
    let a: HashMap<i64, i64> = co.get("a").await?;
    println!("a = {:?}", a);

    co.mutate("a", Map::extend(vb)).await?;
    let a: HashMap<i64, i64> = co.get("a").await?;
    println!("a.extend({:?}) = {:?}", vb, a);

    co.mutate("a", Map::remove(4..)).await?;
    let a: HashMap<i64, i64> = co.get("a").await?;
    println!("a.remove(4..) = {:?}", a);
    co.mutate("a", Map::remove([0, 1])).await?;
    let a: HashMap<i64, i64> = co.get("a").await?;
    println!("a.remove([0, 1]) = {:?}", a);

    let a: i64 = co.select("a", Map::len()).await?;
    println!("a.len() = {:?}", a);
    let a: HashMap<i64, i64> = co.select("a", Map::index(2..)).await?;
    println!("a.index(2..) = {:?}", a);
    let a: HashMap<i64, i64> = co.select("a", Map::index([1, 2])).await?;
    println!("a.index([1, 2]) = {:?}", a);

    Ok(())
}
