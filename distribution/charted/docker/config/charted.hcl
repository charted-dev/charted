# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
# Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# This is the default configuration for `charted-server` when running as
# a Docker container. You can mount the data directory from `/var/lib/noelware/charted/data`
# as a regular filesystem mount with `-v $(pwd)/w:/var/lib/noelware/charted/data`.

database "sqlite" {
  db_path = "/var/lib/noelware/charted/data/charted.db"
}

storage "filesystem" {
  directory = "/var/lib/noelware/charted/data"
}
