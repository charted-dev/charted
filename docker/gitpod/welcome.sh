#!/bin/bash

# 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Go.
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

PINK='\033[38;2;241;204;209m'
RESET='\033[0m'
BOLD='\033[1m'

log() {
    local timestamp=$(date +"%D - %r")
    printf '%b\\n' "${PINK}${timestamp}${RESET} ==> $1"
}

info "Welcome to the ${BOLD}charted-server${RESET} Gitpod container!"
info "   To run ${BOLD}charted-server${RESET}, run \`make build\` to create a local binary,"
info "   Run \`./bin/charted-server generate\` to create a configuration file,"
info "   Run \`./bin/charted-server -c ./config.yoml\` to run the server. :)"
info ""
info "  Happy coding or contributing!"
info "~ Noelware"
