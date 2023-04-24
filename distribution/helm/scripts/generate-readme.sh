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

# This script is here to help generate parameters for our documentation, which
# will live in https://artifacts.noelware.cloud/charted/server/[version]/helm/params.json
# and updated in the README.
#
# At the moment, we don't generate Helm parameters for `values.yaml`, but we will soon
# once we stablised our documentation.

BASH_SRC=${BASH_SOURCE[0]}
while [ -L "$BASH_SRC" ]; do
    target=$(readlink "$BASH_SRC")
    if [[ $target == /* ]]; then
        #debug "source [$BASH_SRC] is an absolute symlink to $target"
        BASH_SRC=$target
    else
        dir=$(dirname "$BASH_SRC")
        #debug "source [$BASH_SRC] is a relative symlink to [$target] (relative -> $dir)"

        BASH_SRC=$dir/$target
    fi
done

REAL_DIR=$(dirname "$BASH_SRC")
DIR=$(cd -P "$(dirname "$BASH_SRC")" >/dev/null 2>&1 && pwd)

# First, we need to check if Node.js exists
if ! command -v node >/dev/null; then
    echo "[charted::generate-readme] You will need to have Node.js installed."
fi

# Check if we have NPM installed, which we shouldn't reach this
# if we have Node.js installed, let's just be safe (for now).
if ! command -v npm >/dev/null; then
    echo "[charted::generate-readme] Missing \`npm\` command, did you install Node.js?"
fi

# Now we need to have Git installed (so we can grab Bitnami Labs' Helm README generator)
if ! command -v git >/dev/null; then
    echo "[charted::generate-readme] Missing \`git\` command."
fi

if ! [ -d "$DIR/../.cache/readme-helm" ]; then
    mkdir $DIR/../.cache
    echo "[charted::generate-readme] Cloning \`https://github.com/bitnami-labs/readme-generator-for-helm\` in .cache/helm-readme"
    git clone https://github.com/bitnami-labs/readme-generator-for-helm .cache/readme-helm

    (cd $DIR/../.cache/readme-helm && npm i)
fi

node $DIR/../.cache/readme-helm/bin -v $DIR/../values.yaml --readme $DIR/../README.md
