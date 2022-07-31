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

# Temurin doesn't support Alpine images for arm64/v8, so Ubuntu will have to do...
# for now~
FROM eclipse-temurin:18.0.1_10-jdk-jammy AS builder

ENV DEBIAN_FRONTEND=noninteractive
ENV PROTOC_VERSION="21.4"
RUN apt update && apt upgrade -y && \
    apt install -y curl git ca-certificates unzip && \
    curl -L -o /tmp/protoc.zip https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/protoc-${PROTOC_VERSION}-linux-aarch_64.zip && \
    mkdir -p /opt/protoc && \
    unzip -d /opt/protoc /tmp/protoc.zip

WORKDIR /build/charted
ENV CHARTED_PROTOC_PATH=/opt/protoc/bin/protoc

COPY . .
RUN chmod +x ./gradlew
RUN ./gradlew :server:installDist --stacktrace

FROM eclipse-temurin:18.0.1_10-jdk-jammy

ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt upgrade -y && apt install -y tini bash

WORKDIR /app/noelware/charted/server

COPY distribution/docker/scripts/linux /app/noelware/charted/server/scripts
COPY --from=builder /build/charted/server/build/install/charted-server/charted-server /app/noelware/charted/server/charted-server
COPY --from=builder /build/charted/server/build/install/charted-server/lib /app/noelware/charted/server/lib

RUN chmod +x /app/noelware/charted/server/scripts/docker-entrypoint.sh && \
    chmod +x /app/noelware/charted/server/charted-server

ENV CHARTED_DISTRIBUTION_TYPE=docker
USER 1001

ENTRYPOINT ["/app/noelware/charted/server/scripts/docker-entrypoint.sh"]
CMD ["/app/noelware/charted/server/charted-server"]
