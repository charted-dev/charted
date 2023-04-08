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

# This script is mainly used by the Linting GitHub action,
# so it will only be applicable to that only.

CONFIG_YAML="
jwt_secret_key: woofbarkbark
database:
    username: charted
    password: charted
    host: postgres
    port: 5432
redis:
    host: redis
"

echo "$CONFIG_YAML" >> ./config.yml
./cli/build/install/charted/bin/charted server &

RETRIES=0
while true; do
    if [ $RETRIES -eq 5 ]; then
        echo "Unable to request to heartbeat endpoint -- retries 5 times and didn't succeed. :("
        exit 1
    fi

    STATUS_CODE=$(curl -sw '%{http_code}' --output /dev/null http://localhost:3651/heartbeat)
    if [[ $STATUS_CODE == "200" ]]; then
        break
    fi

    echo "Received status code [$STATUS_CODE], retrying in 5 seconds [$(( RETRIES++ ))/5]"
    sleep 5
done

rm ./assets/openapi.json
curl -fsSL http://localhost:3651/_openapi?pretty=true | jq . > ./assets/openapi.json
