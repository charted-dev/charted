#!/bin/bash

# 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
# Copyright 2022 Noelware <team@noelware.org>
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

set -o errexit
set -o nounset
set -o pipefail

. /app/charted/server/scripts/liblog.sh

if ! [[ "${CHARTED_ENABLE_WELCOME_PROMPT:-yes}" =~ ^(no|false)$ ]]; then
    info ""
    info "   Welcome to the ${BOLD}charted-server${RESET} container image."
    info "   > Free, open source, and reliable Helm Chart registry made in Kotlin"
    info ""
    info "   Subscribe to the project for more updates: https://github.com/charted-dev/charted"
    info "   Any issues occur? Report it!               https://github.com/charted-dev/charted/issues"
    info ""
fi

tini -s "$@"
