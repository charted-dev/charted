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

. .\liblog.ps1

$ShowBanner = [Environment]::GetEnvironmentVariable("CHARTED_ENABLE_WELCOME_PROMPT") ?? "yes";
if ($ShowBanner -match "^(yes|true|1|si|si*)$") {
    Info ""
    Info "  Welcome to the charted-server Windows container image."
    Info "  📦 Free, open source, and reliable Helm Chart registry made in Kotlin."
    Info ""
    Info "  * Subscribe to the project for updates: https://github.com/charted-dev/charted"
    Info "  * Any issues occur? Report it to us at GitHub: https://github.com/charted-dev/charted/issues"
    Info ""
}
