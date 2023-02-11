#!/bin/bash

# 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

function write! {
  echo $1
}

if ! [ -x "./gradlew" ]; then
  write! "Missing ./gradlew in directory [$PWD]"
  exit 1
fi

write! "[generate:openapi] Compiling CLI subproject..."
./gradlew :cli:installDist

write! "[generate:openapi] Generating document..."
chmod +x ./cli/build/install/charted/bin/charted
PRETTY_RESULT=$(./cli/build/install/charted/bin/charted openapi --format=json | jq -M .)

echo $PRETTY_RESULT > ./assets/openapi.json
write! "[generate:openapi] Done!"
