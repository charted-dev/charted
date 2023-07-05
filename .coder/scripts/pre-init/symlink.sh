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

if ! [ -f "/usr/local/bin/buildifier" ]; then
    echo "===> Symlinking \`bazel-buildifier\` ~> \`buildifier\`!"
    sudo ln -s /usr/local/bin/bazel-buildifier /usr/local/bin/buildifier
fi

if ! [ -f "/usr/local/bin/sync-deps" ]; then
    echo "===> Symlinking ./scripts/sync_deps.sh ~> sync-deps"
    sudo ln -s $HOME/workspace/scripts/sync_deps.sh /usr/local/bin/sync-deps
fi
