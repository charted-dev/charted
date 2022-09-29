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

# This script will allow you to upload Qodana Reports to GitHub and access
# them with the url: https://qodana.noelware.cloud/charted/server/{branch/commit/pr}

# If you're going to use this script, please update this to the repository you're using!
REPO="Noelware/qodana-reports"
REPORTS_DIR=$(mktemp -d)
SUFFIX=""
GIT_USER=${GIT_USER:-}
GIT_EMAIL=${GIT_EMAIL:-}
GIT_TOKEN=${GIT_TOKEN:-GITHUB_TOKEN:-}

if [[ -z "$GIT_USER" || -z "$GIT_EMAIL" || -z "$GIT_TOKEN" ]]; then
  echo "Missing \`GIT_USER\`, \`GIT_EMAIL\`, or \`GIT_TOKEN\` environment variables"
  exit 1
fi

echo "Cloning repository $REPO to $REPORTS_DIR/qodana"
git clone https://$GIT_USER:$GIT_TOKEN@github.com/Noelware/qodana-reports $REPORTS_DIR/qodana -b gh-pages

if [[ $GITHUB_REF == refs/heads/* ]]; then
  SUFFIX=$(echo $GITHUB_REF | sed -e 's/\/.*\///g' -e 's/refs//')
  if [[ "$SUFFIX" == gh-* ]]; then
    SUFFIX="issue/$SUFFIX"
  fi

  echo "Using branch path [charted/web/$SUFFIX]"
elif [[ $GITHUB_REF == refs/prs/* ]]; then
  SUFFIX="pr/$(echo $GITHUB_REF | grep -o '[[:digit:]]' | tr -d '\n')"
  echo "Using PR path [charted/web/$SUFFIX]"
else
  echo "Unable to collect reports path! Skipping..."
  exit 1
fi

echo "Now collecting from Qodana..."
QODANA_REPORTS_DIR=$RUNNER_TEMP/qodana/results/report

mkdir -p $REPORTS_DIR/qodana/charted/server/$SUFFIX
cp -r $QODANA_REPORTS_DIR $REPORTS_DIR/qodana/charted/server/$SUFFIX

git config --global user.email $GIT_EMAIL
git config --global user.name $GIT_USER

cd $REPORTS_DIR/qodana
git add .
git commit -m "Upload charted/server Qodana for JVM artifacts

Referenced from commit: $GITHUB_SHA
https://github.com/charted-dev/charted/commits/$GITHUB_SHA"

git push -u origin gh-pages
rm -rf $REPORTS_DIR
