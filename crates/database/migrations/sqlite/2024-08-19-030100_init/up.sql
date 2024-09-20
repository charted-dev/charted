-- 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
-- Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
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

CREATE TABLE IF NOT EXISTS `users`(
    verified_publisher BOOLEAN NOT NULL DEFAULT false,
    gravatar_email TEXT NULL DEFAULT NULL,
    description VARCHAR(240) NULL DEFAULT NULL,
    avatar_hash TEXT NULL DEFAULT NULL,
    created_at DATETIME NOT NULL DEFAULT(NOW()),
    updated_at DATETIME NOT NULL DEFAULT(NOW()),
    username VARCHAR(64) NOT NULL,
    password TEXT NULL DEFAULT NULL,
    email TEXT NOT NULL,
    admin BOOLEAN NOT NULL DEFAULT false,
    name VARCHAR(64) NULL DEFAULT NULL,
    id TEXT NOT NULL PRIMARY KEY
);

CREATE UNIQUE INDEX idx_users_username ON users(username);
CREATE UNIQUE INDEX idx_users_email ON users(email);

CREATE TABLE IF NOT EXISTS `user_connections`(
    noelware_account_id BIGINT NULL DEFAULT NULL,
    google_account_id TEXT NULL DEFAULT NULL,
    github_account_id TEXT NULL DEFAULT NULL,
    apple_account_id TEXT NULL DEFAULT NULL,
    created_at DATETIME NOT NULL DEFAULT(NOW()),
    updated_at DATETIME NOT NULL DEFAULT(NOW()),
    account TEXT NOT NULL,
    id TEXT PRIMARY KEY NOT NULL,

    CONSTRAINT `fk_user_connections_owner` FOREIGN KEY(account) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS `repositories`(
    description VARCHAR(64) NULL DEFAULT NULL,
    deprecated BOOLEAN NOT NULL DEFAULT false,
    created_at DATETIME NOT NULL DEFAULT(NOW()),
    updated_at DATETIME NOT NULL DEFAULT(NOW()),
    icon_hash TEXT NULL DEFAULT NULL,
    private BOOLEAN NOT NULL DEFAULT false,
    owner TEXT NOT NULL,
    name VARCHAR(32) NOT NULL,
    type TEXT CHECK(type IN ('application', 'library', 'operator')) NOT NULL DEFAULT 'application',
    id TEXT NOT NULL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS `repository_releases`(
    repository TEXT NOT NULL UNIQUE,
    update_text TEXT NULL DEFAULT NULL,
    created_at DATETIME NOT NULL DEFAULT(NOW()),
    updated_at DATETIME NOT NULL DEFAULT(NOW()),
    tag TEXT NOT NULL,
    id TEXT NOT NULL PRIMARY KEY,

    CONSTRAINT `fk_repository_release_owner` FOREIGN KEY(repository) REFERENCES repositories(id)
);

CREATE TABLE IF NOT EXISTS `repository_members`(
    public_visibility BOOLEAN NOT NULL DEFAULT false,
    display_name VARCHAR(32) NULL DEFAULT NULL,
    permissions BIGINT NOT NULL DEFAULT 0,
    repository TEXT NOT NULL,
    updated_at DATETIME NOT NULL DEFAULT(NOW()),
    joined_at DATETIME NOT NULL DEFAULT(NOW()),
    account TEXT NOT NULL,
    id TEXT NOT NULL PRIMARY KEY,

    CONSTRAINT `fk_repository_members_repository_id` FOREIGN KEY(repository) REFERENCES repositories(id),
    CONSTRAINT `fk_repository_members_account_id` FOREIGN KEY(account) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS `organizations`(
    verified_publisher BOOLEAN NOT NULL DEFAULT false,
    twitter_handle TEXT NULL DEFAULT NULL,
    gravatar_email TEXT NULL DEFAULT NULL,
    display_name VARCHAR(32) NULL DEFAULT NULL,
    created_at DATETIME NOT NULL DEFAULT(NOW()),
    updated_at DATETIME NOT NULL DEFAULT(NOW()),
    icon_hash TEXT NULL DEFAULT NULL,
    private BOOLEAN NOT NULL DEFAULT false,
    owner TEXT NOT NULL,
    name VARCHAR(32) NOT NULL UNIQUE,
    id TEXT NOT NULL PRIMARY KEY,

    CONSTRAINT `fk_organization_owner_id` FOREIGN KEY(owner) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS `organization_members`(
    public_visibility BOOLEAN NOT NULL DEFAULT false,
    display_name VARCHAR(32) NULL DEFAULT NULL,
    organization TEXT NOT NULL,
    permissions BIGINT NOT NULL DEFAULT 0,
    updated_at DATETIME NOT NULL DEFAULT(NOW()),
    joined_at DATETIME NOT NULL DEFAULT(NOW()),
    account TEXT NOT NULL,
    id TEXT NOT NULL PRIMARY KEY,

    CONSTRAINT `fk_organization_members_organization_id` FOREIGN KEY(organization) REFERENCES organizations(id),
    CONSTRAINT `fk_organization_members_account_id` FOREIGN KEY(account) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS `api_keys`(
    description VARCHAR(140) NULL DEFAULT NULL,
    created_at DATETIME NOT NULL DEFAULT(NOW()),
    updated_at DATETIME NOT NULL DEFAULT(NOW()),
    expires_in DATETIME NULL DEFAULT NULL,
    scopes BIGINT NOT NULL DEFAULT 0,
    owner TEXT NOT NULL,
    token TEXT NOT NULL,
    name VARCHAR(32) NOT NULL,
    id TEXT NOT NULL PRIMARY KEY,

    CONSTRAINT `fk_api_keys_owner_id` FOREIGN KEY(owner) REFERENCES users(id)
);
