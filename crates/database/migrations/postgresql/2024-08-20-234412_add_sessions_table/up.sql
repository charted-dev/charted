-- 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
-- Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
--
-- Licensed under the Apache License, Version 2.0 (the "License");
-- you may not use this file except in compliance with the License.
-- You may obtain a copy of the License at
--
--    http://www.apache.org/licenses/LICENSE-2.0
--
-- Unless required by applicable law or agreed to in writing, software
-- distributed under the License is distributed on an "AS IS" BASIS,
-- WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
-- See the License for the specific language governing permissions and
-- limitations under the License.

CREATE TABLE IF NOT EXISTS "sessions"(
    refresh_token TEXT NOT NULL,
    access_token TEXT NOT NULL,
    owner TEXT NOT NULL,
    id TEXT NOT NULL PRIMARY KEY,

    CONSTRAINT "fk_sessions_owner" FOREIGN KEY(owner) REFERENCES users(id)
);
