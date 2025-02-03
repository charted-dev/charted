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

mod member;

/*
CREATE TABLE IF NOT EXISTS "organizations"(
    verified_publisher BOOLEAN NOT NULL DEFAULT false,
    twitter_handle TEXT NULL DEFAULT NULL,
    gravatar_email TEXT NULL DEFAULT NULL,
    display_name VARCHAR(32) NULL DEFAULT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT(NOW()),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT(NOW()),
    icon_hash TEXT NULL DEFAULT NULL,
    private BOOLEAN NOT NULL DEFAULT false,
    owner TEXT NOT NULL,
    name VARCHAR(32) NOT NULL UNIQUE,
    id TEXT NOT NULL PRIMARY KEY,

    CONSTRAINT "fk_organization_owner_id" FOREIGN KEY(owner) REFERENCES users(id)
);
*/
