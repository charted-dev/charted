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
mod release;

/*
CREATE TABLE IF NOT EXISTS "repositories"(
    description VARCHAR(64) NULL DEFAULT NULL,
    deprecated BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT(NOW()),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT(NOW()),
    icon_hash TEXT NULL DEFAULT NULL,
    private BOOLEAN NOT NULL DEFAULT false,

    -- `creator` is only null if `owner` is not a *User*.
    creator TEXT NULL DEFAULT NULL,
    owner TEXT NOT NULL,
    name VARCHAR(32) NOT NULL,
    type chart_type NOT NULL DEFAULT('application'),
    id TEXT NOT NULL PRIMARY KEY
);
*/
