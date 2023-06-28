#!/bin/bash

# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

# In small-tight Coder deployments, it is recommended to disable the Gradle
# daemon so we do not leak too much memory from Gradle since IntelliJ does
# take a chunk of memory itself.

echo "===> Disabling Gradle daemon..."
! [ -d "$HOME/.gradle" ] && mkdir -p $HOME/.gradle
if ! [ -f "$HOME/.gradle/gradle.properties" ]; then
    echo "org.gradle.daemon=false" >> "$HOME/.gradle/gradle.properties"
fi
