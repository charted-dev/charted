#!/bin/bash

# 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
# Copyright 2022 Noelware <team@noelware.org>
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

echo "[devcontainer] setting up cassandra database..."

# cqlsh \
#     -u cassandra \
#     -p cassandra \
#     -e "CREATE KEYSPACE IF NOT EXISTS charted WITH REPLICATION = { 'class': 'SimpleStrategy', 'replication_factor': 1 };"

# ./gradlew :tools:migrations:buildx64MigrationsImage
# docker run --rm \
#     --name cassandra-migrations \
#     docker.noelware.org/charted/migrations:latest-amd64
