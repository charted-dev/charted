# 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
# Copyright 2022 Noelware <team@noelware.org>
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
#  Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

#!/bin/bash

set -o errexit
set -o pipefail
set -o nounset

PACKAGE_NAME=charted-server
echo "[charted-server] Publishing v$RELEASE_VERSION..."

export HOME=/home/noel
ssh-keyscan -t ed25519 aur.archlinux.org >> $HOME/.ssh/known_hosts
echo -e "${SSH_PRIVATE_KEY//_/\\n}" > $HOME/.ssh/aur

git config --global user.name "$COMMIT_USERNAME"
git config --global user.email "$COMMIT_EMAIL"
REPO_URL="ssh://aur@aur.archlinux.org/charted-server.git"
echo "[charted-server] Repository URL: $REPO_URL"

cd /tmp
git clone "$REPO_URL"
cd charted-server

echo "[charted-server] Building package..."
sed -i "s/pkgver=.*$/pkgver=$RELEASE_VERSION/" PKGBUILD
sed -i "s/pkgrel=.*$/pkgrel=1/" PKGBUILD
updpkgsums

echo "[charted-server] Edited release version, testing package..."
makepkg -c
echo "[charted-server] Build has finished. :)"

git add PKGBUILD .SRCINFO
git commit --allow-empty -m "chore: update to $RELEASE_VERSION"
git push
