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

FROM --platform=$BUILDPLATFORM golang:1.19.3-alpine3.16 AS migrations

RUN apk update && apk add --no-cache ca-certificates git
WORKDIR /build/migrations

COPY databases/clickhouse/migrations .
RUN go mod tidy && \
    go mod download && \
    go build -ldflags "-s -w" -o ./bin/migrations

FROM --platform=$BUILDPLATFORM eclipse-temurin:17-jdk-alpine AS jdk-runtime

RUN apk update && \
    apk add --no-cache binutils && \
    jlink --add-modules ALL-MODULE-PATH \
            --strip-debug \
            --no-man-pages \
            --no-header-files \
            --compress=2 \
            --output /runtime

FROM --platform=$BUILDPLATFORM eclipse-temurin:17-jdk-alpine AS gradle-build

RUN apk update && apk add --no-cache git ca-certificates gcompat libc6-compat
WORKDIR /build/server

COPY . .
RUN chmod +x ./gradlew && ./gradlew :cli:installDist --no-daemon --stacktrace

FROM --platform=$BUILDPLATFORM alpine:3.16

RUN apk update && apk add --no-cache bash tini libc6-compat gcompat
WORKDIR /app/noelware/charted/server

ENV JAVA_HOME=/opt/openjdk/java
COPY --from=gradle-build /build/cli/build/install/charted/config /app/noelware/charted/server/config
COPY --from=gradle-build /build/cli/build/install/charted/lib    /app/noelware/charted/server/lib
COPY --from=gradle-build /build/cli/build/install/charted/bin    /app/noelware/charted/server/bin
COPY                     distributions/docker/scripts/linux      /app/noelware/charted/server/scripts
COPY --from=migrations   /build/migrations/bin/migrations        /app/noelware/charted/server/bin/ch-migrations
COPY --from=jdk-runtime  /runtime                                /opt/openjdk/java

ENV CHARTED_DISTRIBUTION_TYPE=docker
EXPOSE 3651
VOLUME /var/lib/noelware/charted/data

RUN mkdir -p /var/lib/noelware/charted/data
RUN addgroup -g 1001 charted && \
    adduser -h "/app/noelware/charted/server" -u 1001 -G charted -s /bin/bash -D noelware && \
    chown -R noelware:noelware /app/noelware/charted/server && \
    chown -R noelware:noelware /var/lib/noelware/charted/data && \
    chmod +x /app/noelware/charted/server/bin/charted

USER noelware
ENTRYPOINT ["/app/noelware/charted/server/scripts/docker-entrypoint.sh"]
CMD ["/app/noelware/charted/server/bin/charted", "server"]
