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

use crate::{make_config, var};
use charted_common::TRUTHY_REGEX;

make_config! {
    CdnConfig {
        #[serde(default)]
        pub enabled: bool {
            default: true;
            env_value: var!("CHARTED_CDN_ENABLE", {
                or_else: true;
                mapper: |val| TRUTHY_REGEX.is_match(val.as_str());
            });
        };

        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub prefix: Option<String> {
            default: None;
            env_value: var!("CHARTED_CDN_PREFIX", is_optional: true);
        };
    }
}
