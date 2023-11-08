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

BAZEL=${BAZEL:-bazel}
BAZEL_STARTUP_ARGS=${BAZEL_STARTUP_ARGS:-}
BAZEL_ARGS=${BAZEL_ARGS:-}

if [[ "x$BAZEL" == "xbazel" ]] && ! command -v bazel >/dev/null; then
    echo "===> [packaging.sh]: unable to find \`bazel\` binary!"
    exit 1
fi

echo "===> Building web distribution..."
echo "===> $ $BAZEL $BAZEL_STARTUP_ARGS build $BAZEL_ARGS //web:build" | xargs

$BAZEL $BAZEL_STARTUP_ARGS build $BAZEL_ARGS //web:build
cp "$SCRIPT_DIR/bazel-bin/web/dist" "$SCRIPT_DIR/server/dist"

echo "===> Creating tar archive..."
echo "===> $ $BAZEL $BAZEL_STARTUP_ARGS build --compilation_mode=opt $BAZEL_ARGS --@rules_rust//:extra_rustc_flag=\"--cfg=bundle_web\" //distribution:tarball" | xargs
$BAZEL $BAZEL_STARTUP_ARGS build \
    --compilation_mode=opt \
    --@rules_rust//:extra_rustc_flag="--cfg=bundle_web" \
    "$BAZEL_ARGS" \
    //distribution:tarball

echo "===> Creating zip archive..."
echo "===> $ $BAZEL $BAZEL_STARTUP_ARGS build --compilation_mode=opt $BAZEL_ARGS --@rules_rust//:extra_rustc_flag=\"--cfg=bundle_web\" //distribution:zip" | xargs
$BAZEL $BAZEL_STARTUP_ARGS build \
    --compilation_mode=opt \
    --@rules_rust//:extra_rustc_flag="--cfg=bundle_web" \
    "$BAZEL_ARGS" \
    //distribution:zip
