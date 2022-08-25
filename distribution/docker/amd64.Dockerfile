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

FROM eclipse-temurin:18-jdk-alpine AS jdk-runtime

RUN apk update && \
    apk add --no-cache binutils && \
    jlink --add-modules ALL-MODULE-PATH \
            --strip-debug \
            --no-man-pages \
            --no-header-files \
            --compress=2 \
            --output /runtime

FROM eclipse-temurin:18-jdk-alpine AS gradle-build

RUN apk update && \
    apk add --no-cache git ca-certificates && \
    apk add --no-cache protobuf-dev --repository=https://dl-cdn.alpinelinux.org/alpine/edge/main

WORKDIR /build

# https://github.com/google/protobuf-gradle-plugin/issues/265#issuecomment-421508779
ENV CHARTED_PROTOC_PATH=protoc

COPY . .
RUN chmod +x ./gradlew && ./gradlew :server:installDist --stacktrace

FROM alpine:3.16

RUN apk update && apk add --no-cache bash tini libc6-compat gcompat
WORKDIR /app/noelware/charted/server

ENV JAVA_HOME="/opt/openjdk/java"
COPY --from=jdk-runtime /runtime /opt/openjdk/java
COPY --from=gradle-build /build/build/install/charted-server/lib /app/noelware/charted/server/lib
COPY --from=gradle-build /build/build/install/charted-server/charted-server /app/noelware/charted/server/bin/charted-server
COPY distribution/docker/scripts/linux /app/noelware/charted/server/scripts

ENV CHARTED_DISTRIBUTION_TYPE=docker
EXPOSE 3651

RUN chown 1001:1001 /app/noelware/charted/server && \
    addgroup -g 1001 charted && \
    adduser -h "/app/noelware/charted/server" -u 1001 -G charted -s /bin/bash -D noelware

USER charted
ENTRYPOINT ["/app/noelware/charted/server/scripts/docker-entrypoint.sh"]
CMD ["/app/noelware/charted/server/charted-server"]
