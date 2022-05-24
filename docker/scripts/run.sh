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

join() {
    local IFS="$1"
    shift
    echo "$*"
}

DAEMONIZE=false
ORIGINAL_ARG_ARRAY=("$@")
while [ $# -gt 0 ]; do
  if [[ $1 == "-d" || $1 == "--daemon" ]]; then
    DAEMONIZE=true
  fi

  if [[ $# -gt 0 ]]; then
    shift
  fi
done

info "*** starting charted-server ***"

debug "Dedicated Node ===> ${WINTERFOX_DEDI_NODE:-unknown}"
debug "Logback Config ===> ${CHARTED_LOGBACK_PATH:-unknown}"
debug "JVM Arguments  ===> ${CHARTED_JAVA_OPTS:-unknown}"

RESOLVED_JAVA_OPTS=("-XX:+HeapDumpOnOutOfMemoryError" "-Dfile.encoding=UTF-8")

if [[ -n "${CHARTED_LOGBACK_PATH:-}" && -f "${CHARTED_LOGBACK_PATH}" ]]; then
  RESOLVED_JAVA_OPTS+=("-Dorg.noelware.charted.logback.config=$CHARTED_LOGBACK_PATH")
fi

if [[ -n "${WINTERFOX_DEDI_NODE:-}" ]]; then
  RESOLVED_JAVA_OPTS+=("-Pwinterfox.dediNode=$WINTERFOX_DEDI_NODE")
fi

if [[ -n "${CHARTED_JAVA_OPTS:-}" ]]; then
  RESOLVED_JAVA_OPTS+=($CHARTED_JAVA_OPTS)
fi

export JAVA_OPTS=$(join ' ' "${RESOLVED_JAVA_OPTS[@]}")
debug "Resolved JVM arguments: $JAVA_OPTS"

# Determine the Java command to use.
if [ -n "$JAVA_HOME" ]; then
  if [ -x "$JAVA_HOME/jre/sh/java" ]; then
    JAVA_EXEC=$JAVA_HOME/jre/sh/java
  else
    JAVA_EXEC=$JAVA_HOME/bin/java
  fi

  if [ ! -x "$JAVA_EXEC" ]; then
    fatal "The home path for Java was set to an invalid directory: $JAVA_HOME

Please set the location of the JAVA_HOME environment variable to match the location
of the Java installation."

    exit 1
  fi
else
  JAVA_EXEC=java
  which java >/dev/null 2>&1 || fatal "The JAVA_HOME environment variable was not set and no 'java' command can be
found in the current PATH.

Please set the location of the JAVA_HOME environment variable to match the location
of the Java installation."
  exit 1
fi

CHARTED_CLASSPATH=$(find /app/charted/server/lib -type f -maxdepth 1 | sed -e ':a' -e 'N' -e '$!ba' -e 's/\n/;/g')

if [[ $DAEMONIZE = true ]]; then
  exec \
    "$JAVA_EXEC" \
    "$JAVA_OPTS" \
    -Dorg.noelware.charted.distribution.type="docker" \
    -cp "$CHARTED_CLASSPATH" \
    org.noelware.charted.Bootstrap &

  exit_code=$?
  if [ $exit_code -eq 0 ]; then
    debug "Server has exited with a successful exit code."
    exit 0
  else
    debug "Server has exited with a non-successful exit code of $exit_code."
    exit $exit_code
  fi
else
  exec \
    "$JAVA_EXEC" \
    "$JAVA_OPTS" \
    -Dorg.noelware.charted.distribution.type="docker" \
    -cp "$CHARTED_CLASSPATH" \
    org.noelware.charted.Bootstrap
fi
