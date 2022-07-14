# 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
# Copyright 2022 Noelware <team@noelware.org>
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
#  Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

FROM eclipse-temurin:18-jdk-alpine AS builder

RUN apk update && apk add --no-cache git ca-certificates libc6-compat gcompat && apk add --no-cache protobuf-dev --repository=https://dl-cdn.alpinelinux.org/alpine/edge/main
WORKDIR /build/charted

# https://github.com/google/protobuf-gradle-plugin/issues/265#issuecomment-421508779
ENV CHARTED_PROTOC_PATH=protoc

COPY . .
RUN chmod +x ./gradlew
RUN ./gradlew :server:installDist --stacktrace --scan

FROM eclipse-temurin:18-jdk-alpine

RUN apk update && apk add --no-cache tini bash libc6-compat gcompat
WORKDIR /app/noelware/charted/server

COPY distribution/docker/scripts/linux /app/noelware/charted/server/scripts
COPY --from=builder /build/charted/server/build/install/charted-server/charted-server .
COPY --from=builder /build/charted/server/build/install/charted-server/lib .

RUN chmod +x /app/noelware/charted/server/scripts/docker-entrypoint.sh && \
    chmod +x /app/noelware/charted/server/charted-server

USER 1001

ENTRYPOINT ["/app/noelware/charted/server/scripts/docker-entrypoint.sh"]
CMD ["/app/noelware/charted/server/charted-server"]
