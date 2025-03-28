#!/usr/bin/env bash

# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
# Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
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

set -eo pipefail

# define common functions that we will use
charted::os() {
    case "$(uname -s)" in
        Darwin)
            echo "darwin"
            ;;

        Linux)
            echo "linux"
            ;;

        *)
            echo "Unsupported operating system: \`$(uname -s)\`" >&2
            exit 1
            ;;
    esac
}

charted::architecture() {
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

charted::fetch() {
    local url="$1"

    if charted::command-exists "curl"; then
        curl -fsSL "$url"
    elif [ charted::command-exists "wget" ]; then
        wget -q -S -O - "$url" 2>&1
    else
        charted::error "failed to fetch url \`$url\` as \`curl\` or \`wget\` isn't installed on system"
    fi
}

charted::download() {
    local url="$1"
    local location="$2"

    if charted::command-exists "curl"; then
        curl -SL --progress-bar -o "$location" "$url"
    elif charted::command-exists "wget"; then
        wget "$url" -O "$location" 2>&1
    else
        charted::error "failed to download url \`$url\` to location \`$loc\` as \`curl\` or \`wget\` isn't installed on system"
    fi
}

blue=''
green=''
pink=''
reset=''
bold=''
underline=''
red=''
yellow=''

if [[ -t 1 ]]; then
    blue='\033[38;2;81;81;140m'
    green='\033[38;2;165;204;165m'
    pink='\033[38;2;241;204;209m'
    reset='\033[0m'
    bold='\033[1m'
    underline='\033[4m'
    red='\033[38;166;76;76m'
    yellow='\033[38;227;227;172m'
fi

charted::error() {
    echo -e "${red}error${reset}:" "$@" >&2
}

charted::fatal() {
    echo -e "${red}${bold}fatal${reset}${reset}:" "$@" >&2
    exit 1
}

charted::info() {
    echo -e "${green}info${reset}:" "$@"
}

charted::warn() {
    echo -e "${yellow}warn${reset}:" "$@"
}

charted::command-exists() {
    command -v "$1" >/dev/null
}

main() {
    charted::info "current system: $(charted::os) $(charted::architecture)"

    version=""
    if [ -z "$1" ]; then
        version=$(charted::fetch "https://artifacts.noelware.org/charted/server/versions.json" | jq '.latest.version' | tr -d '"')
        charted::info "fetched latest version: $version"
    else
        version="$1"
    fi

    charted::info "downloading \`charted-$(charted::os)-$(charted::architecture)\` from artifacts server..."
    binary_url="https://artifacts.noelware.org/charted/server/$version/charted-$(charted::os)-$(charted::architecture)"
    checksums_url="https://artifacts.noelware.org/charted/server/$version/charted-$(charted::os)-$(charted::architecture).sha256"

    charted::info "distribution url => $binary_url"
    charted::info "checksum url     => $checksums_url"

    tmpdir=$(mktemp -d)
    tmploc="$tmpdir/charted-$(charted::os)-$(charted::architecture)"

    charted::download "$binary_url" "$tmploc"

    if charted::command-exists "sha256sum"; then
        checksum=$(sha256sum "$tmploc" | awk '{print $1}')
        server_checksum=$(charted::fetch "$checksums_url")
        if ! grep -q "$checksum" "$server_checksum"; then
            charted::error "failed to compute checksum for binary $tmploc"
            charted::error "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"
            charted::error "generated checksum by system:"
            charted::error "$checksum"
            charted::error "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"
            charted::error "checksum by artifacts server:"
            charted::error "server_checksum"
            echo "" 2>&1

            rm -rf "$tmpdir" || {
                charted::warn "failed to cleanup $tmpdir -- artifacts might still exist"
                true
            }

            charted::fatal "exiting."
        fi

        # TODO(@auguwu): --install-dir/CHARTED_INSTALL_DIR?
        mv "$tmploc" "/usr/local/bin/charted"
        charted::info "\`charted\` binary is now installed in \`/usr/local/bin/charted\`."

        exit 0
    fi

    if [ "$(charted::os)" == "darwin" ] && charted::command-exists "shasum"; then
        checksum=$(shasum -256 "$tmploc" | awk '{print $1}')
        server_checksum=$(charted::fetch "$checksums_url")
        if ! grep -q "$checksum" "$server_checksum"; then
            charted::error "failed to compute checksum for binary $tmploc"
            charted::error "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"
            charted::error "generated checksum by system:"
            charted::error "$checksum"
            charted::error "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"
            charted::error "checksum by artifacts server:"
            charted::error "server_checksum"
            echo "" 2>&1

            rm -rf "$tmpdir" || {
                charted::warn "failed to cleanup $tmpdir -- artifacts might still exist"
                true
            }

            charted::fatal "exiting."
        fi

        # TODO(@auguwu): --install-dir/CHARTED_INSTALL_DIR?
        # TODO(@auguwu): is this ok on macos? probably idk
        mv "$tmploc" "/usr/local/bin/charted"
        charted::info "\`charted\` binary is now installed in \`/usr/local/bin/charted\`."

        exit 0
    fi

    charted::fatal "couldn't find \`sha256sum\` or \`shasum\` to compute checksums"
}

main "$@"
