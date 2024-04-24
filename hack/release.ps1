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

# This script is the same as `release.sh` but for Windows.

function Main {
    if (![System.Environment]::Is64BitOperatingSystem) {
        Write-Error "FATAL: 'ume' is not supported on x86 systems!"
        Exit 1
    }

    $Cargo = [System.Environment]::GetEnvironmentVariable('CARGO') || "cargo"
    if (!(Get-Command "$Cargo" -errorAction SilentlyContinue)) {
        Write-Error "FATAL: -Cargo flag was not set to a valid 'cargo' binary"
        exit 1
    }

    # create .result directory as the release workflow requires it
    New-Item -Path . -Name ".result" -ItemType Directory

    $BuildFlags = [System.Environment]::GetEnvironmentVariable('BUILDFLAGS')
    $Args = [System.Environment]::GetEnvironmentVariable('CARGOFLAGS')
    $Binary = [System.Environment]::GetEnvironmentVariable('BINARY')

    Write-Host "$ $Cargo build --release --locked $BuildFlags $Args"
    iex "$Cargo build --release --locked $BuildFlags $Args"
    if (!$?) {
        Write-Error "Failed to run 'cargo build', exiting early"
        exit 1
    }

    # Move ./target/release/{bin}.exe ~> ./.result/{bin}.exe
    Move-Item -Path "./target/release/$Binary.exe" -Destination "./.result/$Binary-windows-x86_64.exe"

    Push-Location ./.result
    (Get-FileHash -Path "$Binary-windows-x86_64.exe").Hash.ToLower() | Out-File "$Binary-windows-x86_64.exe.sha256"

    Pop-Location

    Write-Host "Completed."
}

Main
