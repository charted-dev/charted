# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
# Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
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

###### BINARY BUILD
FROM rustlang/rust:nightly-alpine3.20 AS build

RUN apk upgrade && apk add --no-cache \
    git                               \
    mold                              \
    ca-certificates                   \
    musl-dev                          \
    libc6-compat                      \
    gcompat                           \
    pkgconfig                         \
    openssl-dev                       \
    build-base

WORKDIR /build
COPY . .

# We want to use the Nightly version of Rust from the image itself, not what we
# defined in `rust-toolchain.toml`.
RUN rm rust-toolchain.toml

ENV RUSTFLAGS="--cfg tokio_unstable -C link-arg=-fuse-ld=mold -Ctarget-cpu=native -Ctarget-feature=-crt-static"
RUN cargo build --locked --release --package charted --features bundled-sqlite --features bundled-pq

##### FINAL STAGE
FROM alpine:3.21

RUN apk upgrade && apk add --no-cache \
    bash                              \
    tini                              \
    curl

WORKDIR /app/noelware/charted/server

COPY --from=build /build/target/release/charted /app/noelware/charted/server/bin/charted
COPY distribution/charted/docker/scripts        /app/noelware/charted/server/scripts
COPY distribution/charted/docker/config         /app/noelware/charted/server/config

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
