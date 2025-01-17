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

use std::collections::VecDeque;

use engula_apis::v1::*;

use crate::{Error, Result};

pub struct Args(VecDeque<Value>);

impl Args {
    pub fn new(args: Vec<Value>) -> Self {
        Self(args.into())
    }

    pub fn take<T: TryFrom<Value>>(&mut self) -> Result<T> {
        let v = self
            .0
            .pop_front()
            .ok_or_else(|| Error::invalid_argument("missing argument"))?;
        v.try_into()
            .map_err(|_| Error::invalid_argument("argument type mismatch"))
    }
}
