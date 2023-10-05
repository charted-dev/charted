#!/bin/bash

# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
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

BASH_SRC=${BASH_SOURCE[0]}
while [ -L "$BASH_SRC" ]; do
    target=$(readlink "$BASH_SRC")
    if [[ $target == /* ]]; then
        BASH_SRC=$target
    else
        dir=$(dirname "$BASH_SRC")
        BASH_SRC=$dir/$target
    fi
done

SCRIPT_DIR=$(cd -P "$(dirname $BASH_SRC)/.." >/dev/null 2>&1 && pwd)
NPM_TOKEN=${NPM_TOKEN:-}

if [ "x${NPM_TOKEN}" == "x" ]; then
    echo "===> [publish.sh] missing required \`NPM_TOKEN\` environment variable"
    exit 1
fi

if ! command -v node >/dev/null; then
    echo "===> [publish.sh] missing \`node\` binary, please install Node.js"
    exit 1
fi

version=""
if [ "x${VERSION:-}" != "x" ]; then
    echo "===> Publishing @ncharts/types@${VERSION}"
    version=${VERSION}
fi

if [[ version == "" ]]; then
    echo "===> Publishing @ncharts/types@$(cat "$SCRIPT_DIR/.charted-version")"

    version=$(cat "$SCRIPT_DIR/.charted-version")
fi

echo "$version"
