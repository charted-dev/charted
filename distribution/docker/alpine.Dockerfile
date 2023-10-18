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

# TODO(@auguwu): replace all bazel-related compilation to cr.floofy.dev/bazelbuild/bazel
#                Docker image

FROM --platform=${TARGETPLATFORM} adoptopenjdk/openjdk11:alpine AS bazelbuild

ARG BAZEL_VERSION="6.3.0"
ENV BAZEL_DIST="https://github.com/bazelbuild/bazel/releases/download/${BAZEL_VERSION}/bazel-${BAZEL_VERSION}-dist.zip"

RUN apk update && apk add --no-cache bash build-base python3 zip unzip curl protobuf linux-headers
RUN curl -Lo bazel.zip ${BAZEL_DIST}
RUN unzip -qd /build bazel.zip

WORKDIR /build
ENV EXTRA_BAZEL_ARGS="--tool_java_runtime_version=local_jdk"
ENV JAVA_HOME=/opt/java/openjdk
ENV JAVA_VERSION=11

RUN ./compile.sh || exit 1

FROM --platform=${TARGETPLATFORM} adoptopenjdk/openjdk11:alpine AS bazel

RUN apk update && apk add --no-cache bash libstdc++ build-base tar unzip zip
COPY --from=bazelbuild /build/output/bazel /usr/bin/bazel

FROM bazel AS web

WORKDIR /build

COPY . .

RUN rm /build/.bazelversion
RUN bazel build --compilation_mode=opt //web:build
RUN bazel shutdown

FROM bazel AS build

RUN apk update && apk add --no-cache openssl-dev pkgconfig git ca-certificates
WORKDIR /build

COPY . .
COPY --from=web /build/bazel-bin/web/dist /build/server/dist

RUN rm /build/.bazelversion
RUN bazel build --platforms //build/platforms:linux_x64_musl \
    --compilation_mode=opt \
    --@rules_rust//:extra_rustc_flag="-C target-cpu=haswell" \
    --@rules_rust//:extra_rustc_flag="--cfg=bundle_web" \
    //cli

RUN bazel shutdown

FROM alpine:3.18

RUN apk update && apk add --no-cache bash tini curl
WORKDIR /app/noelware/charted/server

COPY --from=build /build/output/charted             /app/noelware/charted/server/bin/charted
COPY --from=build /build/crates/database/migrations /app/noelware/charted/server/migrations
COPY distribution/docker/scripts                    /app/noelware/charted/server/scripts
COPY distribution/docker/config                     /app/noelware/charted/server/config

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
