#!/usr/bin/env bash

# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
# Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
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

set -eu

if [ -n "${CHARTED_HELM_NO_INSTALL_HOOK:-}" ]; then
    echo "===> In development mode, not downloading!"
    exit 0
fi

function charted::helm::os {
    case "$(uname -s)" in
        Darwin)
            echo "darwin"
            ;;

        Linux)
            echo "linux"
            ;;

        CYGWIN*|MINGW*|MSYS_NT*)
            echo "windows"
            ;;

        *)
            echo "Unsupported operating system: \`$(uname -s)\`" >&2
            exit 1
            ;;
    esac
}

function charted::helm::architecture {
    case "$(uname -m)" in
        x86_64|amd64)
            echo "x86_64"
            ;;

        aarch64|arm64)
            echo "arm64"
            ;;

        *)
            echo "Unsupported architecture: \`$(uname -m)\`" >&2
            exit 1
            ;;
    esac
}

function charted::helm::binary_url {
    local version="$1"

    arch=$(charted::helm::architecture)
    os=$(charted::helm::os)

    if [ "$os" == "windows" ]; then
        echo "https://artifacts.noelware.cloud/charted/helm-plugin/$version/helm-plugin-$os-$arch.exe"
    else
        echo "https://artifacts.noelware.cloud/charted/helm-plugin/$version/helm-plugin-$os-$arch"
    fi
}

function charted::helm::checksum_url {
    local version="$1"

    arch=$(charted::helm::architecture)
    os=$(charted::helm::os)

    if [ "$os" == "windows" ]; then
        echo "https://artifacts.noelware.cloud/charted/helm-plugin/$version/helm-plugin-$os-$arch.exe.sha256"
    else
        echo "https://artifacts.noelware.cloud/charted/helm-plugin/$version/helm-plugin-$os-$arch.sha256"
    fi
}

function charted::helm::download {
    local url="$1"
    local loc="$2"

    if command -v curl >/dev/null; then
        curl -sSL "$url" -o "$loc"
    elif command -v wget >/dev/null; then
        wget -q "$1" -O "$2"
    else
        echo "~> FATAL: Failed to download \`charted-helm-plugin\` as \`curl\` or \`wget\` was not found on the system" >/dev/stderr
    fi
}

function charted::helm::on_exit {
    code=$?
    if [ $code -ne 0 ]; then
        echo "~> ERROR: failed to install \`charted-helm-plugin\`. Please run \`helm plugin uninstall charted\` and try installing again"
    fi

    exit $?
}

trap charted::helm::on_exit EXIT

version="$(grep 'version' plugin.yaml | cut -d "'" -f 2)"
BINARY_URL=$(charted::helm::binary_url "$version")
CHECKSUM_URL=$(charted::helm::checksum_url "$version")

echo "~> Downloading \`charted-helm-plugin\` from binary URL: $BINARY_URL"
charted::helm::download "$BINARY_URL" bin/charted-helm-plugin

echo "~> Downloading \`charted-helm-plugin\` checksum: $CHECKSUM_URL"
charted::helm::download "$CHECKSUM_URL" bin/charted-helm-plugin.sha256

# Verify the checksum
if command -v sha256sum >/dev/null; then
    checksum=$(sha256sum bin/charted-helm-plugin | awk '{print $1}')
    if ! grep -q "$checksum" bin/charted-helm-plugin.sha256; then
        echo "~> FATAL: received invalid sha256 checksum for \`bin/charted-helm-plugin\`"
        exit 1
    fi

    # if it completed, then remove 'bin/charted-helm-plugin.sha256' and continue
    rm bin/charted-helm-plugin.sha256
    exit 0
fi

if [ "$(charted::helm::os)" == "darwin" ]; then
    # i am blaming spotlight if this goes so wrong
    checksum=$(shasum -256 bin/charted-helm-plugin | awk '{print $1}')
    if ! grep -q "$checksum" bin/charted-helm-plugin.sha256; then
        echo "~> FATAL: received invalid sha256 checksum for \`bin/charted-helm-plugin\`"
        exit 1
    fi

    # if it completed, then remove 'bin/charted-helm-plugin.sha256' and continue
    rm bin/charted-helm-plugin.sha256
    exit 0
fi
