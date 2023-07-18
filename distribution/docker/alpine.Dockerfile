# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
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

FROM cr.floofy.dev/noel/bazel:6-alpine AS web

RUN apk update && apk add --no-cache git ca-certificates
WORKDIR /build

COPY . .
RUN bazel build //web:vite_build_release --compilation_mode=fastbuild
RUN mkdir -p /build/output && cp $(bazel cquery //cli:release_bin --output=files &>/dev/null | grep --color=never bazel-out) /build/output/charted
RUN bazel shutdown

FROM cr.floofy.dev/noel/bazel:6-alpine AS build

RUN apk update && apk add --no-cache git ca-certificates
WORKDIR /build

COPY . .
RUN bazel build //cli:release_bin --compilation_mode=fastbuild
RUN mkdir -p /build/output && cp $(bazel cquery //cli:release_bin --output=files &>/dev/null | grep --color=never bazel-out) /build/output/charted
RUN bazel shutdown

FROM alpine:3.18

RUN apk update && apk add --no-cache bash tini curl
WORKDIR /app/noelware/charted/server

COPY --from=build /build/crates/database/migrations /app/noelware/charted/server/migrations
COPY --from=build /build/output/charted             /app/noelware/charted/server/bin/charted
COPY distribution/docker/scripts                    /app/noelware/charted/server/scripts
COPY distribution/config                            /app/noelware/charted/server/config

ENV CHARTED_DISTRIBUTION_TYPE=docker
EXPOSE 3651
VOLUME /var/lib/noelware/charted/data

RUN mkdir -p /var/lib/noelware/charted/data
RUN addgroup -g 1001 noelware && \
    adduser -DSH -u 1001 -G noelware noelware && \
    chown -R noelware:noelware /app/noelware/charted/server && \
    chown -R noelware:noelware /var/lib/noelware/charted/data && \
    chmod +x /app/noelware/charted/server/bin/charted /app/noelware/charted/server/scripts/docker-entrypoint.sh

# Create a symbolic link so you can just run `charted` without specifying
# the full path.
RUN ln -s /app/noelware/charted/server/bin/charted /usr/bin/charted

USER noelware
ENTRYPOINT ["/app/noelware/charted/server/scripts/docker-entrypoint.sh"]
CMD ["/app/noelware/charted/server/bin/charted", "server"]
