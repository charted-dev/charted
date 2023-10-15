#!/usr/bin/env bash

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

# This script updates the TypeScript version and integrity and the Node.js version + checksums
# in an automated way.

if ! command -v curl >/dev/null; then
    echo ">> This script requires the use of cURL, please install it"
    exit 1
fi

if ! command -v grep >/dev/null; then
    echo ">> This script requires grep! Please install it."
fi

if ! command -v jq >/dev/null; then
    echo ">> This script requires the use of jq, please install it"
    exit 1
fi

if ! command -v tr >/dev/null; then
    echo ">> This script requires the use of the 'tr' command, please install it"
    exit 1
fi

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
if ! [ -f "$SCRIPT_DIR/build/tools/nodejs.bzl" ]; then
    echo ">> You must be in charted's root directory to find the required \`build/tools/nodejs.bzl\` file."
    exit 1
fi

TYPESCRIPT_VERSION=${TYPESCRIPT_VERSION:-"5.2.2"}
NODE_VERSION=${NODE_VERSION:-"20.8.0"}

echo ">> Updating Node.js version to $NODE_VERSION"
sed -i -e "s,NODE_VERSION = \".*\",NODE_VERSION = \"${NODE_VERSION}\",g" "$SCRIPT_DIR/build/tools/nodejs.bzl"

echo ">> Updating TypeScript's NPM integrity..."
integrity=$(curl -s https://registry.npmjs.org/typescript | jq ".versions.[\"$TYPESCRIPT_VERSION\"].dist.integrity" | tr -d '"')

echo ">> Updating Starlark constants:"
echo ">>     ~> TYPESCRIPT_VERSION   ~> \"$TYPESCRIPT_VERSION\""
echo ">>     ~> TYPESCRIPT_INTEGRITY ~> \"$integrity\""

sed -i -e "s,TYPESCRIPT_INTEGRITY = \".*\",TYPESCRIPT_INTEGRITY = \"${integrity}\",g" "$SCRIPT_DIR/build/tools/nodejs.bzl"
sed -i -e "s,TYPESCRIPT_VERSION = \".*\",TYPESCRIPT_VERSION = \"${TYPESCRIPT_VERSION}\",g" "$SCRIPT_DIR/build/tools/nodejs.bzl"

NODE_OS=(
    "linux"
    "darwin"
    "win"
)

NODE_ARCH=(
    "x64"
    "arm64"
)

checksums=$(curl -fsSL "https://nodejs.org/dist/v${NODE_VERSION}/SHASUMS256.txt.asc")

for os in "${NODE_OS[@]}"; do
    for arch in "${NODE_ARCH[@]}"; do
        if [[ "$os" == "win" && "$arch" == "arm64" ]]; then
            break
        fi

        ext=""
        case "$os" in
            linux)
                ext=".tar.xz"
                ;;

            darwin)
                ext=".tar.gz"
                ;;
            win)
                ext=".zip"
                ;;
            *)
                break
                ;;
        esac

        final_arch="$arch"
        final_os="$os"
        case "$arch" in
            x64)
                final_arch="amd64"
                ;;
            *)
                ;;
        esac

        case "$os" in
            win)
                final_os="windows"
                ;;
            *)
                ;;
        esac

        echo ">> Updating Node.js checksums for $os/$arch for Node.js $NODE_VERSION"
        checksum=$(echo "$checksums" | grep "node-v${NODE_VERSION}-$os-$arch$ext" | awk '{printf $1}')
        if [ $? != "0" ]; then
            echo ">>     ~> Skipping entry due to grep failing with code $?"
            continue
        fi

        key=$(printf '%s:%s' "$final_os" "$final_arch")
        sed -i -e "s,    \"$key\": \".*\",    \"$key\": \"$checksum\",g" "$SCRIPT_DIR/build/tools/nodejs.bzl"
    done
done

echo ">> Updated checksums for Node.js ${NODE_VERSION}"
