#!/bin/bash

# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
# Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

mkdir ~/.persist

SERVICES=$(docker compose -f docker-compose.yml config --format=json | jq '.services')
SERVICE_KEYS=$(echo "$SERVICES" | jq 'keys[]' | tr -d '"')

for key in $SERVICE_KEYS; do
    echo "===> Creating persistence directory in [~/.persist/$key]"
    mkdir ~/.persist/"$key"

    if [ "$key" == "postgres" ]; then
        sudo chown -R 1001:1001 ~/.persist/postgres
    fi

    if [ "$key" == "redis" ]; then
        sudo chown -R 1001:1001 ~/.persist/redis
    fi
done
