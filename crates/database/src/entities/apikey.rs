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

/*
CREATE TABLE IF NOT EXISTS "api_keys"(
    description VARCHAR(140) NULL DEFAULT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT(NOW()),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT(NOW()),
    expires_in TIMESTAMP WITH TIME ZONE NULL DEFAULT NULL,
    scopes BIGINT NOT NULL DEFAULT 0,
    owner TEXT NOT NULL,
    token TEXT NOT NULL,
    name VARCHAR(32) NOT NULL,
    id TEXT NOT NULL PRIMARY KEY,

    CONSTRAINT "fk_api_keys_owner_id" FOREIGN KEY(owner) REFERENCES users(id)
);
*/
