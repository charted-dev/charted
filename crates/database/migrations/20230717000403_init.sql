/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
 * Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

CREATE OR REPLACE FUNCTION updated_entity_timestamp()
RETURNS TRIGGER AS $body$
    BEGIN
        NEW.updated_at = NOW();
        return NEW;
    END
$body$
LANGUAGE plpgsql;

-- Create users table
CREATE TABLE IF NOT EXISTS users(
    verified_publisher BOOLEAN NOT NULL DEFAULT false,
    gravatar_email TEXT NULL DEFAULT null,
    description VARCHAR(240) NULL DEFAULT null,
    avatar_hash TEXT NULL DEFAULT null,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT(NOW()),
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT(NOW()),
    username VARCHAR(64) NOT NULL,
    password TEXT NULL DEFAULT NULL,
    email TEXT NOT NULL,
    admin BOOLEAN NOT NULL DEFAULT false,
    name VARCHAR(64) NULL DEFAULT NULL,
    id BIGINT NOT NULL PRIMARY KEY
);

CREATE UNIQUE INDEX idx_users_username ON users(username);
CREATE UNIQUE INDEX idx_users_email ON users(email);
CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE PROCEDURE updated_entity_timestamp();

CREATE TABLE IF NOT EXISTS user_connections(
    noelware_account_id BIGINT NULL DEFAULT NULL,
    google_account_id TEXT NOT NULL DEFAULT NULL,
    github_account_id TEXT NOT NULL DEFAULT NULL,
    apple_account_id TEXT NOT NULL DEFAULT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT(NOW()),
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT(NOW()),
    id BIGINT PRIMARY KEY NOT NULL
);

CREATE TRIGGER update_users_connections_updated_at
    BEFORE UPDATE ON user_connections
    FOR EACH ROW EXECUTE PROCEDURE updated_entity_timestamp();

CREATE TYPE chart_type AS ENUM('application', 'library', 'operator');
CREATE TABLE IF NOT EXISTS repositories(
    description VARCHAR(64) NULL DEFAULT NULL,
    deprecated BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT(NOW()),
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT(NOW()),
    icon_hash TEXT NULL DEFAULT NULL,
    private BOOLEAN NOT NULL DEFAULT false,
    owner BIGINT NOT NULL,
    name VARCHAR(32) NOT NULL,
    type chart_type NOT NULL DEFAULT('application'),
    id BIGINT NOT NULL PRIMARY KEY
);

CREATE UNIQUE INDEX idx_repositories_owner_id ON repositories(owner);
CREATE UNIQUE INDEX idx_repositories_name ON repositories(name);
CREATE TRIGGER update_repositories_updated_at
    BEFORE UPDATE ON repositories
    FOR EACH ROW EXECUTE PROCEDURE updated_entity_timestamp();

CREATE TABLE IF NOT EXISTS repository_releases(
    repository BIGINT NOT NULL UNIQUE REFERENCES repositories(id),
    update_text TEXT NULL DEFAULT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT(NOW()),
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT(NOW()),
    tag TEXT NOT NULL,
    id BIGINT NOT NULL PRIMARY KEY
);

ALTER TABLE repository_releases
ADD CONSTRAINT fk_repository_release_owner
FOREIGN KEY(repository) REFERENCES repositories(id);

CREATE TRIGGER update_repository_releases_updated_at
BEFORE UPDATE ON repository_releases
FOR EACH ROW EXECUTE PROCEDURE updated_entity_timestamp();

CREATE TABLE IF NOT EXISTS repository_members(
    public_visibility BOOLEAN NOT NULL DEFAULT false,
    display_name VARCHAR(32) NULL DEFAULT NULL,
    permissions BIGINT NOT NULL DEFAULT 0,
    repository BIGINT NOT NULL REFERENCES repositories(id),
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT(NOW()),
    joined_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT(NOW()),
    account BIGINT NOT NULL REFERENCES users(id),
    id BIGINT NOT NULL PRIMARY KEY
);

ALTER TABLE repository_members
ADD CONSTRAINT fk_repository_members_repository_id
FOREIGN KEY(repository) REFERENCES repositories(id);

ALTER TABLE repository_members
ADD CONSTRAINT fk_repository_members_account_id
FOREIGN KEY(account) REFERENCES users(id);

CREATE TRIGGER update_repository_members_updated_at
BEFORE UPDATE ON repository_members
FOR EACH ROW EXECUTE PROCEDURE updated_entity_timestamp();

CREATE TABLE IF NOT EXISTS organizations(
    verified_publisher BOOLEAN NOT NULL DEFAULT false,
    twitter_handle TEXT NULL DEFAULT NULL,
    gravatar_email TEXT NULL DEFAULT NULL,
    display_name VARCHAR(32) NULL DEFAULT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT(NOW()),
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT(NOW()),
    icon_hash TEXT NULL DEFAULT NULL,
    private BOOLEAN NOT NULL DEFAULT false,
    owner BIGINT NOT NULL REFERENCES users(id),
    name VARCHAR(32) NOT NULL UNIQUE,
    id BIGINT NOT NULL PRIMARY KEY
);

ALTER TABLE organizations
ADD CONSTRAINT fk_organization_owner_id
FOREIGN KEY(owner) REFERENCES users(id);

CREATE UNIQUE INDEX idx_organizations_name ON organizations(name);

CREATE TRIGGER update_organizations_updated_at
BEFORE UPDATE ON organizations
FOR EACH ROW EXECUTE PROCEDURE updated_entity_timestamp();

CREATE TABLE IF NOT EXISTS organization_members(
    public_visibility BOOLEAN NOT NULL DEFAULT false,
    display_name VARCHAR(32) NULL DEFAULT NULL,
    organization BIGINT NOT NULL REFERENCES organizations(id),
    permissions BIGINT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT(NOW()),
    joined_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT(NOW()),
    account BIGINT NOT NULL REFERENCES users(id),
    id BIGINT NOT NULL PRIMARY KEY
);

ALTER TABLE organization_members
ADD CONSTRAINT fk_organization_members_organization_id
FOREIGN KEY(organization) REFERENCES repositories(id);

ALTER TABLE organization_members
ADD CONSTRAINT fk_organization_members_account_id
FOREIGN KEY(account) REFERENCES users(id);

CREATE TRIGGER update_organization_members_updated_at
BEFORE UPDATE ON organization_members
FOR EACH ROW EXECUTE PROCEDURE updated_entity_timestamp();

CREATE TABLE IF NOT EXISTS api_keys(
    description VARCHAR(140) NULL DEFAULT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT(NOW()),
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT(NOW()),
    expires_in TIMESTAMP WITHOUT TIME ZONE NULL DEFAULT NULL,
    scopes BIGINT NOT NULL DEFAULT 0,
    owner BIGINT NOT NULL REFERENCES users(id),
    token TEXT NOT NULL,
    name VARCHAR(32) NOT NULL,
    id BIGINT NOT NULL PRIMARY KEY
);

ALTER TABLE api_keys
ADD CONSTRAINT fk_api_keys_owner_id
FOREIGN KEY(owner) REFERENCES users(id);

CREATE TRIGGER update_api_keys_updated_at
BEFORE UPDATE ON api_keys
FOR EACH ROW EXECUTE PROCEDURE updated_entity_timestamp();
