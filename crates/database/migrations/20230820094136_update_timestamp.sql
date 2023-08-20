/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
 * Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

ALTER TABLE api_keys ALTER COLUMN created_at TYPE TIMESTAMP WITH TIME ZONE;
ALTER TABLE api_keys ALTER COLUMN updated_at TYPE TIMESTAMP WITH TIME ZONE;

ALTER TABLE organizations ALTER COLUMN created_at TYPE TIMESTAMP WITH TIME ZONE;
ALTER TABLE organizations ALTER COLUMN updated_at TYPE TIMESTAMP WITH TIME ZONE;

ALTER TABLE organization_members ALTER COLUMN joined_at TYPE TIMESTAMP WITH TIME ZONE;
ALTER TABLE organization_members ALTER COLUMN updated_at TYPE TIMESTAMP WITH TIME ZONE;

ALTER TABLE repositories ALTER COLUMN created_at TYPE TIMESTAMP WITH TIME ZONE;
ALTER TABLE repositories ALTER COLUMN updated_at TYPE TIMESTAMP WITH TIME ZONE;

ALTER TABLE repository_members ALTER COLUMN joined_at TYPE TIMESTAMP WITH TIME ZONE;
ALTER TABLE repository_members ALTER COLUMN updated_at TYPE TIMESTAMP WITH TIME ZONE;

ALTER TABLE repository_releases ALTER COLUMN created_at TYPE TIMESTAMP WITH TIME ZONE;
ALTER TABLE repository_releases ALTER COLUMN updated_at TYPE TIMESTAMP WITH TIME ZONE;

ALTER TABLE users ALTER COLUMN created_at TYPE TIMESTAMP WITH TIME ZONE;
ALTER TABLE users ALTER COLUMN updated_at TYPE TIMESTAMP WITH TIME ZONE;

ALTER TABLE user_connections ALTER COLUMN created_at TYPE TIMESTAMP WITH TIME ZONE;
ALTER TABLE user_connections ALTER COLUMN updated_at TYPE TIMESTAMP WITH TIME ZONE;
