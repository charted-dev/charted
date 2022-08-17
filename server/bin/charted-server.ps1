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

[CmdletBinding()]
Param(
    [String]$ConfigPath = "",
    [Boolean]$EnableElasticAPM = $false
)

function Fatal {
    Param(
        [String]$Message
    )

    Write-Error "FATAL :: $Message"
    exit 1
}

function FindViaRegistry {
    foreach ($k in @('Java Development Kit', 'Java Runtime Environment')) {
        $key = Join-Path "HKLM:\Software\JavaSoft" $k
        if (-not $(Test-Path $key)) {
            continue
        }

        $current = $(Get-Item $key).GetValue('CurrentVersion')
        if (-not $current) {
            continue
        }

        $home = $(Get-Item $(Join-Path $key $current)).GetValue('JavaHome')
        if ($home) {
            $java = Get-Command -ErrorAction SilentlyContinue $(Join-Path $home "bin/java.exe")
            if ($java) {
                Write-Host "[preinit] Found Java via Windows Registry ($key)"
                return $java
            }
        }
    }

    return $null
}

$DistributionType = [Environment]::GetEnvironmentVariable("CHARTED_DISTRIBUTION") ?? "local";
$ResolvedJavaOpts = $(
    "-Dfile.encoding=UTF-8",
    "-Djava.awt.headless=true",
    "-XX:+HeapDumpOnOutOfMemoryError",
    "-XX:+ExitOnOutOfMemoryError",
    "-XX:ErrorFile=logs/hs_err_pid%p.log",
    "-XX:+UseConcMarkSweepGC",
    "-XX:+CMSParallelRemarkEnabled",
    "-XX:SurvivorRatio=8",
    "-XX+CMSParallelInitialMarkEnabled",
    "-XX:+CMSEdenChunksRecordAlways",
    "-XX:+UseCMSInitiatingOccupancyOnly"
)

if ([Environment]::GetEnvironmentVariable("CHARTED_LOGBACK_PATH")) {
    $ResolvedJavaOpts.Add("-Dorg.noelware.charted.logback.config=$Env:CHARTED_LOGBACK_PATH")
}

if ([Environment]::GetEnvironmentVariable("WINTERFOX_DEDI_NODE")) {
    $ResolvedJavaOpts.Add("-Pwinterfox.dediNode=$Env:WINTERFOX_DEDI_NODE")
}

if ([Environment]::GetEnvironmentVariable("CHARTED_JAVA_OPTS")) {
    $ResolvedJavaOpts.Add($Env:CHARTED_JAVA_OPTS)
}

if ($EnableElasticAPM == $true) {
    $ResolvedJavaOpts.Add("-Dorg.noelware.charted.tracing=apm")
}

$JavaOpts = $ResolvedJavaOpts -join " "
Write-Host "[preinit] Resolved JAVA_OPTS ===> $JavaOpts"

$JavaExec = ""
if ($env:JAVA_HOME) {
    $ext = if ($IsMacOS -or $IsLinux) { "" } else { ".exe" }
    $java = Get-Command -ErrorAction SilentlyContinue $(Join-Path $env:JAVA_HOME "bin/java$ext")
    if ($java) {
        Write-Host "[preinit] Found Java via JAVA_HOME ($env:JAVA_HOME)"
        $JavaExec = $java
    }
} else {
    # Check if we can just use `java[.exe]` instead
    $ext = if ($IsMacOS -or $IsLinux) { "" } else { ".exe" }
    $java1 = Get-Command "java$ext" -ErrorAction SilentlyContinue
    if ($java1) {
        Write-Host "[preinit] Found Java via PATH"
        $JavaExec = $java1
    }

    if ($IsWindows) {
        $java2 = FindViaRegistry
        if ($java2) {
            $JavaExec = $java2
        }
    }

    if ($JavaExec -eq "") {
        Fatal "Unable to find Java installation from JAVA_HOME environment variable, system path! Please install JDK17 or higher."
    }
}

$Classpath = $(Get-ChildItem -Recurse "$PSScriptRoot/lib" -Include *.jar -ErrorAction SilentlyContinue -Force | ForEach-Object FullName) -join ":"

& "$JavaExec" $JavaOpts -Dorg.noelware.charted.distribution.type="$DistributionType" -cp "$Classpath" org.noelware.charted.server.Bootstrap
exit $LASTEXITCODE
