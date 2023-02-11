#!/bin/bash

# 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

# This script installs Bitnami's Helm README generator, which is available at
# https://github.com/bitnami-labs/readme-generator-for-helm
#
# This will install the generator in ./.dist/helm-generator

CURRENT_DIR=$(pwd)

if [ ! -d "$CURRENT_DIR/.dist/helm-generator" ]; then
  echo "[::helm-readme] Creating directory $CURRENT_DIR/.dist"
  mkdir -p $CURRENT_DIR/.dist/helm-generator

  git clone https://github.com/bitnami-labs/readme-generator-for-helm $CURRENT_DIR/.dist/helm-generator
  cd $CURRENT_DIR/.dist/helm-generator && npm i && cd ../..
else
  echo "[::helm-readme] Generator already exists in $CURRENT_DIR/.dist/helm-generator, skipping installation"
fi

node .dist/helm-generator/bin -v $CURRENT_DIR/values.yaml --readme $CURRENT_DIR/README.md
