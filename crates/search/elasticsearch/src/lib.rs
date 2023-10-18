// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// it's not finished :)
#![allow(dead_code, unused_imports)]

use charted_common::hashmap;
use charted_config::{elasticsearch::AuthType, Config, SearchConfig};
use elasticsearch::{
    auth::Credentials,
    http::{
        headers,
        transport::{SingleNodeConnectionPool, TransportBuilder},
    },
    Elasticsearch,
};
use eyre::{eyre, Result};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use tracing::info;
use url::Url;

static MAPPINGS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    hashmap! {
        "charted-organizations" => include_str!("../mappings/charted-organizations.json"),
        "charted-repositories" => include_str!("../mappings/charted-repositories.json"),
        "charted-users" => include_str!("../mappings/charted-users.json")
    }
});

/// Represents the search options of Elasticsearch.
#[derive(Debug, Clone)]
pub struct SearchOptions {
    allow_partial_data: bool,
    filter: Vec<String>,
    sealed: bool,
    offset: usize,
    limit: usize,
}

impl charted_search::SearchOptions for SearchOptions {
    fn allow_partial(&mut self, allow: bool) -> &mut Self {
        if self.sealed {
            panic!("INTERNAL BUG: cannot apply filter due to this SearchOptions being sealed");
        }

        self.allow_partial_data = allow;
        self
    }

    fn filter<I: Into<String>>(&mut self, filter: I) -> &mut Self {
        if self.sealed {
            panic!("INTERNAL BUG: cannot apply filter due to this SearchOptions being sealed");
        }

        self.filter.push(filter.into());
        self
    }

    fn limit(&mut self, limit: usize) -> &mut Self {
        if self.sealed {
            panic!("INTERNAL BUG: cannot apply filter due to this SearchOptions being sealed");
        }

        self.limit = limit;
        self
    }

    fn offset(&mut self, offset: usize) -> &mut Self {
        if self.sealed {
            panic!("INTERNAL BUG: cannot apply filter due to this SearchOptions being sealed");
        }

        self.offset = offset;
        self
    }

    fn seal(&mut self) -> Self {
        Self {
            allow_partial_data: self.allow_partial_data,
            filter: self.filter.clone(),
            offset: self.offset,
            sealed: true,
            limit: self.limit,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ElasticsearchService {
    client: Elasticsearch,
}

impl ElasticsearchService {
    pub fn new(_config: charted_config::elasticsearch::Config) -> Result<ElasticsearchService> {
        Err(eyre!("not implemented yet"))
    }
}
