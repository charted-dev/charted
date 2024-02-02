# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
# Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
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

<#
.SYNOPSIS
    A installer script for `charted` for Windows users.
.DESCRIPTION
    This is a single PowerShell script that allows Windows users to install the
    `charted` binary from Noelware's Artifacts Registry with a single line:

        $ irm https://i.noel.pink/charted.ps1 | iex
.NOTES
    Copyright 2022-2024 Noelware, LLC. ~ Released under the Apache 2.0 License
    Read the LICENSE (https://charts.noelware.org/oss/license) for more information.
.PARAMETER Help
    Returns the help menu for the `charted` PowerShell installer
.PARAMETER ArtifactUrl
    URI to where to download artifacts from. This will default to `https://artifacts.noelware.cloud`,
    but it can be customised.
.PARAMETER NeverModifyPath
    Allows the script to never modify your `$PATH` environment variable to include the `charted` binary.
#>

param(
    [Parameter(HelpMessage = "Returns the help menu for the `charted` PowerShell installer")]
    [switch]$Help,
    [Parameter(HelpMessage = "Allows the script to never modify your `$PATH` environment variable to include the `charted` binary.")]
    [switch]$NeverModifyPath = $False,
    [Parameter(HelpMessage = "URI to where to download artifacts from. This will default to `https://artifacts.noelware.cloud`, but it can be customised.")]
    $ArtifactUrl = 'https://artifacts.noelware.cloud'
)

# Define constants
$APP = "charted"
$VERSION = "{{Version}}"

function Main() {
    if (![System.Environment]::Is64BitOperatingSystem) {
        Write-Error "FATAL: `charted` is not supported on x86 systems!"
        Exit 1
    }

    if ($Help) {
        Get-Help $PSCommandPath -Detailed
        Exit
    }

    Write-Host "[installer::INFO] Now installing $APP v$VERSION"
    $url = formatArtifactUri($ArtifactUrl)

    Write-Verbose "URL: $url"
}

function formatArtifactUri($artifact) {
    $arch = normalizeOSArchitecture
    return "$artifact/charted/server/$VERSION/charted-windows-$arch.exe"
}

function normalizeOSArchitecture() {
    return "x86_64"
}

Main
