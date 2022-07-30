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

function Info {
    param(
        [string]$Message
    )

    Write-Host "$([char]0x1b)[38;2;165;204;165m$([char]0x1b)[1minfo $([char]0x1b)[0m   ~ $Message"
}

function Err {
    param(
        [string]$Message
    )

    Write-Host "$([char]0x1b)[38;2;166;76;76m$([char]0x1b)[1merror $([char]0x1b)[0m  ~ $Message"
}

function Warn {
    param(
        [string]$Message
    )

    Write-Host "$([char]0x1b)[38;2;233;233;130m$([char]0x1b)[1mwarn $([char]0x1b)[0m   ~ $Message"
}

function Debug {
    param(
        [string]$Message
    )

    $Enabled = [Environment]::GetEnvironmentVariable('CHARTED_DEBUG')
    if ($Enabled -match "^(yes|true|1|si)$") {
        Write-Host "$([char]0x1b)[38;2;81;81;140m$([char]0x1b)[1mdebug $([char]0x1b)[0m  ~ $Message"
    }
}
