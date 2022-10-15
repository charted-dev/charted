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

FROM node:18.11-alpine3.16 AS builder

RUN apk update && apk add --no-cache git ca-certificates
WORKDIR /build

COPY package.json .
COPY .yarn/ ./.yarn/
COPY .yarnrc.yml .

ENV NODE_ENV=production
RUN yarn install

COPY . .
RUN TS_NODE_PROJECT=./scripts/tsconfig.json yarn build

# We could use NGINX but caddy is simple :3
FROM caddy:2.6.2-alpine

WORKDIR /app/noelware/charted/web

COPY --from=builder /build/dist /app/noelware/charted/web
COPY distribution/docker/Caddyfile /etc/caddy/Caddyfile
