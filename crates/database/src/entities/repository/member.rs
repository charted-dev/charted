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

// CREATE TABLE IF NOT EXISTS "repository_members"(
// public_visibility BOOLEAN NOT NULL DEFAULT false,
// display_name VARCHAR(32) NULL DEFAULT NULL,
// permissions BIGINT NOT NULL DEFAULT 0,
// repository TEXT NOT NULL,
// updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT(NOW()),
// joined_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT(NOW()),
// account TEXT NOT NULL,
// id TEXT NOT NULL PRIMARY KEY,
//
// CONSTRAINT "fk_repository_members_repository_id" FOREIGN KEY(repository) REFERENCES
// repositories(id), CONSTRAINT "fk_repository_members_account_id" FOREIGN KEY(account)
// REFERENCES users(id) );
//
