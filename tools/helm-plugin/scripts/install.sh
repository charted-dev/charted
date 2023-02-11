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

if [ -n "${NO_INSTALL_HOOK}" ]; then
    echo "[charted-helm] Not running install hook!"
    exit 0
fi

STACK_VERSION_TUPLE=$(cat plugin.yaml | grep version | tr -d '"')
STACK_VERSION=${STACK_VERSION_TUPLE//version:/   }
echo "[charted-helm] Now installing with stack version v$STACK_VERSION!"

# The backup download URL will grab the releases from GitHub releases if artifacts.noelware.cloud
# is down.
BACKUP_DOWNLOAD_URL=""
DOWNLOAD_URL=""
OS_ARCH=""

case $(uname -m) in
    x86_64)
        OS_ARCH="amd64"
        ;;
    aarch64|arm64)
        OS_ARCH="arm64"
        ;;
    *)
        echo "[charted-helm] Architecture [$(uname -m)] is not supported."
        exit 1
        ;;
esac

if [ "$(uname)" = "Darwin" ]; then
    DOWNLOAD_URL="https://artifacts.noelware.cloud/charted/server/$STACK_VERSION/helm-plugin-darwin-${OS_ARCH}.tar.gz"
    BACKUP_DOWNLOAD_URL="https://github.com/charted-dev/charted/releases/download/v$STACK_VERSION/helm-plugin-darwin-${OS_ARCH}.tar.gz"
elif [ "$(uname)" = "Linux" ]; then
    DOWNLOAD_URL="https://artifacts.noelware.cloud/charted/server/$STACK_VERSION/helm-plugin-linux-${OS_ARCH}.tar.gz"
    BACKUP_DOWNLOAD_URL="https://github.com/charted-dev/charted/releases/download/v$STACK_VERSION/helm-plugin-linux-${OS_ARCH}.tar.gz"
else
    DOWNLOAD_URL="https://artifacts.noelware.cloud/charted/server/$STACK_VERSION/helm-plugin-windows-${OS_ARCH}.tar.gz"
    BACKUP_DOWNLOAD_URL="https://github.com/charted-dev/charted/releases/download/v$STACK_VERSION/helm-plugin-windows-${OS_ARCH}.tar.gz"
fi

echo "[charted-helm] Installing from $DOWNLOAD_URL (backup url: $BACKUP_DOWNLOAD_URL)"
mkdir -p bin releases/v$STACK_VERSION

if [ -x "$(which curl 2>/dev/null)" ]; then
    curl -sSL ${DOWNLOAD_URL} -o "releases/$STACK_VERSION.tar.gz"
else
    wget -q ${DOWNLOAD_URL} -O "releases/$STACK_VERSION.tar.gz"
fi

tar xzf releases/$STACK_VERSION.tar.gz -C "releases/v$STACK_VERSION"
mv releases/$STACK_VERSION/bin/charted-helm bin/charted-helm || mv releases/$STACK_VERSION/bin/charted-helm.exe bin/charted-helm
