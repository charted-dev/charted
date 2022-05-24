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

# Create the builder stage, where charted-server is built from.
FROM eclipse-temurin:18-jdk-alpine AS builder

# Install git, which is required for metadata.
RUN apk update && apk add --no-cache git ca-certificates

# Set the working directory to /build/charted
WORKDIR /build/charted

# Copy everything from $ROOT to the image.
COPY . .

# Make `./gradlew` an runnable executable and build!
RUN chmod +x ./gradlew && ./gradlew :server:installDist --stacktrace --no-daemon

# This is the runtime image, where we can execute as the container.
FROM eclipse-temurin:18-jdk-alpine

# Installs bash, which the Docker scripts we use are in Bash.
# And, we'll install `tini`, which is a valid "init" for containers. :D
#
# Also, since we use Netty, we need a glibc -> musl compatibility layer.
RUN apk update && apk add --no-cache tini bash libc6-compat gcompat

# Sets the working directory to `/app/charted/server`
WORKDIR /app/charted/server

# Copy our Docker scripts
COPY docker/scripts /app/charted/server/scripts

# Get all the libraries charted-server relies on.
COPY --from=builder /build/charted/server/build/install/charted/lib /app/charted/server/lib

# Copy `config/` to the root directory
COPY --from=builder /build/charted/server/build/install/charted/config /app/charted/server/config

# Copy the executable that is used to run the server.
COPY --from=builder /build/charted/server/build/install/charted/bin/server /app/charted/server/charted-server

# Make sure the Docker scripts are executable
RUN chmod +x /app/charted/serverscripts/docker-entrypoint.sh /app/charted/server/scripts/run.sh

# Security reasons. :)
USER 1001

# Set the entrypoint and runner.
ENTRYPOINT ["/app/charted/server/scripts/docker-entrypoint.sh"]
CMD ["/app/charted/server/scripts/run.sh"]
