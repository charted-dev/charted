# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
# Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
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
FROM rustlang/rust:nightly-alpine3.21 AS build

RUN apk upgrade && apk add --no-cache \
    git                               \
    mold                              \
    ca-certificates                   \
    musl-dev                          \
    openssl-dev                       \
    pkgconfig                         \
    build-base

WORKDIR /build
COPY . .

# We want to use the Nightly toolchain that is provided by the image
# itself and not what we have (this will eliminate most of the `components`
# section, which is fine since we don't need them for a simple build)
RUN rm rust-toolchain.toml

# We also need `rust-src` so we can build `libstd` as well.
RUN rustup component add rust-src

# It might be a bad choice but we decided to not opt into `cargo-chef` since
# releases aren't being pushed as frequently so cache will be stale either way
# and the compute we have *should* not take 5-6 hours.
ENV RUSTFLAGS="--cfg tokio_unstable -Clink-arg=-fuse-ld=mold -Ctarget-cpu=native -Ctarget-feature=-crt-static"
RUN cargo build                                                               \
    -Z build-std=std,panic_abort                                              \
    -Z build-std-features="optimize_for_size,panic_immediate_abort,backtrace" \
    --locked                                                                  \
    --release                                                                 \
    --no-default-features                                                     \
    --bin charted

##### FINAL STAGE
FROM alpine:3.21

RUN apk upgrade && apk add --no-cache \
    bash                              \
    tini                              \
    curl                              \
    libgcc

WORKDIR /app/noelware/charted/server

COPY --from=build /build/target/release/charted /app/noelware/charted/server/bin/charted
COPY distribution/docker/scripts                /app/noelware/charted/server/scripts
COPY distribution/docker/config                 /app/noelware/charted/server/config

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
