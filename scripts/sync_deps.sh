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

BAZEL=${BAZEL:-bazel}
BAZEL_ARGS=${BAZEL_ARGS:-}
BAZEL_STARTUP_ARGS=${BAZEL_STARTUP_ARGS:-}

# Check if we have Bazel installed
# TODO(@auguwu): allow the option to download it.
if [[ "x$BAZEL" == "xbazel" ]] && ! command -v bazel >/dev/null; then
    echo "[sync-deps] ==> unable to find \`bazel\` binary!"
fi

echo "===> Syncing dependencies..."
echo "===> $ $BAZEL $BAZEL_STARTUP_ARGS run $BAZEL_ARGS //thirdparty:crate_index -- --repin" | xargs

CARGO_BAZEL_REPIN=all $BAZEL $BAZEL_STARTUP_ARGS run $BAZEL_ARGS //thirdparty:crate_index -- --repin

echo "===> Patching \`openssl-sys\` and \`curl-sys\`..."

patch -u thirdparty/crates/BUILD.openssl-sys-0.9.93.bazel -i ./build/patches/0002-patch-openssl-sys.patch
patch -u thirdparty/crates/BUILD.curl-sys-0.4.65+curl-8.2.1.bazel -i ./build/patches/0003-curl-sys.patch