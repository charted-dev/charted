#!/bin/bash

echo "[::charted-server] Checking Spotless..."

./gradlew spotlessCheck
ret_val=$?

if [[ "$ret_val" != "0" ]]; then
  echo "[::charted-server] Run \`make spotless\` and commit again."
  exit $ret_val
else
  echo "[::charted-server] 👍 Everything seems ok!"
fi
