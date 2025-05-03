// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
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

use super::*;

#[test]
fn sanity_check_if_all_tags_are_correct() {
    let doc = Document::openapi();

    // we already know we have tags already -- we shouldn't have to add a
    // panic path here.
    let tags = unsafe { doc.tags.unwrap_unchecked() };

    for (path, item) in doc.paths.paths {
        for (method, op) in [
            ("get", item.get),
            ("put", item.put),
            ("post", item.post),
            ("patch", item.patch),
            ("delete", item.delete),
        ] {
            let Some(op) = op else {
                continue;
            };

            if let Some(optags) = op.tags {
                for tag in optags {
                    assert!(
                        tags.iter().any(|x| x.name == tag),
                        "operation {method} {path}: tag '{tag}' doesn't exist in openapi document"
                    );
                }
            }
        }
    }
}
