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

-- Create session table
CREATE TABLE IF NOT EXISTS sessions(
    refresh_token TEXT NOT NULL,
    access_token TEXT NOT NULL,
    expires_in TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT(NOW()),
    user_id BIGINT NOT NULL UNIQUE REFERENCES users(id),
    id UUID NOT NULL PRIMARY KEY
);

ALTER TABLE sessions
ADD CONSTRAINT fk_session_user
FOREIGN KEY(user_id) REFERENCES users(id);

-- Create a function where for every insert, PostgreSQL will delete
-- old entries; it'll delete 4 day old sessions (since 21/06/24, charted-server
-- used to do 7 days for refresh tokens, but 4 days seems like a better limit)
--
-- credit: https://stackoverflow.com/questions/26046816/is-there-a-way-to-set-an-expiry-time-after-which-a-data-entry-is-automaticall#26063344
CREATE FUNCTION delete_old_sessions() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    DELETE FROM sessions where expires_in < NOW() - INTERVAL '4 days';
    RETURN NEW;
END;
$$;

CREATE TRIGGER delete_old_sessions_trigger
AFTER INSERT ON sessions
EXECUTE PROCEDURE delete_old_sessions();
