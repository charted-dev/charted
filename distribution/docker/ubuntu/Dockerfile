# 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
# Copyright 2022-2023 Noelware <team@noelware.org>
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

FROM --platform=$BUILDPLATFORM eclipse-temurin:17.0.6_10-jdk-jammy AS jdk-runtime

ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && \
    jlink --add-modules ALL-MODULE-PATH \
            --strip-debug \
            --no-man-pages \
            --no-header-files \
            --compress=2 \
            --output /runtime

FROM --platform=$BUILDPLATFORM eclipse-temurin:17.0.6_10-jdk-jammy AS gradle-build

ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt install -y git ca-certificates curl
WORKDIR /build/server

COPY . .
RUN chmod +x ./gradlew && ./gradlew :cli:installDist --no-daemon --stacktrace

FROM --platform=$BUILDPLATFORM ubuntu:jammy

RUN apt update && apt upgrade -y && apt install -y bash tini
WORKDIR /app/noelware/charted/server

ENV JAVA_HOME=/opt/openjdk/java
COPY --from=gradle-build /build/server/cli/build/install/charted/config /app/noelware/charted/server/config
COPY --from=gradle-build /build/server/cli/build/install/charted/lib    /app/noelware/charted/server/lib
COPY --from=gradle-build /build/server/cli/build/install/charted/bin    /app/noelware/charted/server/bin
COPY                     distribution/docker/scripts/linux              /app/noelware/charted/server/scripts
COPY --from=jdk-runtime  /runtime                                       /opt/openjdk/java

# Remove the PowerShell script (since it's useless on *UNIX)
RUN rm /app/noelware/charted/server/bin/charted.ps1

ENV CHARTED_DISTRIBUTION_TYPE=docker
EXPOSE 3651
VOLUME /var/lib/noelware/charted/data

RUN mkdir -p /var/lib/noelware/charted/data
RUN groupadd -g 1001 noelware && \
  useradd -rm -s /bin/bash -g noelware -u 1001 noelware && \
  chown 1001:1001 /app/noelware/charted/server && \
  chown 1001:1001 /var/lib/noelware/charted/data && \
  chmod +x /app/noelware/charted/server/bin/charted /app/noelware/charted/server/scripts/docker-entrypoint.sh

# Create a symbolic link so you can just run `charted` without specifying
# the full path.
RUN ln -s /app/noelware/charted/server/bin/charted /usr/bin/charted

USER noelware
ENTRYPOINT ["/app/noelware/charted/server/scripts/docker-entrypoint.sh"]
CMD ["/app/noelware/charted/server/bin/charted", "server"]
