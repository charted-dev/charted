/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022 Noelware <team@noelware.org>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/*****************************************************************
* The initialization SQL file for ClickHouse to create the available
* tables and such.
*
* # From Git repository
*   $ clickhouse-client -f ./init.sql
*
* # Docker Image (charted/init-audit-logs)
*   $ docker run --rm -e CLICKHOUSE_HOST=localhost -e CLICKHOUSE_PORT=9000 -e CLICKHOUSE_DATABASE=charted \
*       -e CLICKHOUSE_USERNAME=username-to-authenticate \
*       -e CLICKHOUSE_PASSWORD=password-to-authenticate \
*       charted/init-audit-logs:<server version>
*
* # Docker Image (./Dockerfile)
*   $ docker buildx build . -t charted/init-audit-logs:latest
*   $ docker run --rm -e CLICKHOUSE_HOST=localhost -e CLICKHOUSE_PORT=9000 -e CLICKHOUSE_DATABASE=charted \
*       -e CLICKHOUSE_USERNAME=username-to-authenticate \
*       -e CLICKHOUSE_PASSWORD=password-to-authenticate \
*       charted/init-audit-logs:<server version>
*****************************************************************/

DROP TABLE IF EXISTS charted.audit_logs;

SET allow_experimental_object_type = 1;
CREATE TABLE "charted.audit_logs"(
    FiredAt DateTime64,
    ID UInt64,
    Action enum('repo.modify', 'repo.starred', 'repo.unstarred', 'repo.pull', 'repo.push', 'org.modify', 'org.new_member', 'org.updated_member', 'org.remove_member'),
    Data JSON,
    OriginID UInt64,
    OriginType enum('repo', 'org')
) ENGINE = MergeTree() PARTITION BY toYYYYMM(FiredAt) ORDER BY (Action, FiredAt, ID);

-- Uncomment this out for ZooKeeper Replication.
--CREATE TABLE "charted.audit_logs"(
--    FiredAt DateTime64,
--    ID UInt64,
--    Action enum('repo.modify', 'repo.deleted', 'repo.starred', 'repo.unstarred', 'repo.pull', 'repo.push', 'org.modify', 'org.new_member', 'org.updated_member', 'org.remove_member'),
--    Data JSON,
--    ver UInt16
--) ENGINE = ReplicatedMergeTree('/clickhouse/tables/{layer}-{shard}/audit-logs', '{replica}', ver) PARTITION BY toYYYYMM(FiredAt) ORDER BY (Action, FiredAt, ID);
