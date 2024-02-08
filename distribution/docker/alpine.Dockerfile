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

############ FRONTEND

FROM --platform=${TARGETPLATFORM} oven/bun:1.0-alpine AS web

RUN apk update && apk add --no-cache git ca-certificates
WORKDIR /build

COPY web/package.json .
COPY web/bun.lockb .

RUN bun install --frozen-lockfile
COPY web .

RUN bun run build

############ BINARY

FROM --platform=${TARGETPLATFORM} rust:1.76-alpine3.19 AS build

# We use the `protobuf` package instead of `protobuf-dev` since we vendor `google/protobuf` in the `protos/`
# directory and we use `prost-types` which Prost does on each release for the well-known google.protobuf.* types.
RUN apk update && apk add --no-cache git ca-certificates curl musl-dev libc6-compat gcompat pkgconfig openssl-dev build-base protobuf
WORKDIR /build

COPY . .
COPY --from=web /build/dist /build/web/dist

# Remove the `rust-toolchain.toml` file since we expect to use `rustc` from the Docker image
# rather from rustup.
RUN rm rust-toolchain.toml

ENV RUSTFLAGS="--cfg tokio_unstable --cfg bundle_web -Ctarget-cpu=native -Ctarget-feature=-crt-static"
RUN cargo build --locked --release

############ FINAL STAGE

FROM alpine:3.19

# We need a hard dependency on `libgcc` since it is required for `--panic=unwind`. We could experiment
# with `--panic=abort`, but I'm not too sure if that works with `color_eyre`!
#
# > Why we need `libgcc`:
# /app/noelware/charted/server$ ldd bin/charted
#         /lib/ld-musl-x86_64.so.1 (0x7fb1f8d26000)
#         libssl.so.3 => /lib/libssl.so.3 (0x7fb1f73ef000)
#         libcrypto.so.3 => /lib/libcrypto.so.3 (0x7fb1f6fd2000)
# Error loading shared library libgcc_s.so.1: No such file or directory (needed by bin/charted)
#         libc.musl-x86_64.so.1 => /lib/ld-musl-x86_64.so.1 (0x7fb1f8d26000)
# Error relocating bin/charted: _Unwind_Resume: symbol not found
# Error relocating bin/charted: _Unwind_GetRegionStart: symbol not found
# Error relocating bin/charted: _Unwind_SetGR: symbol not found
# Error relocating bin/charted: _Unwind_GetDataRelBase: symbol not found
# Error relocating bin/charted: _Unwind_DeleteException: symbol not found
# Error relocating bin/charted: _Unwind_GetLanguageSpecificData: symbol not found
# Error relocating bin/charted: _Unwind_RaiseException: symbol not found
# Error relocating bin/charted: _Unwind_FindEnclosingFunction: symbol not found
# Error relocating bin/charted: _Unwind_GetIP: symbol not found
# Error relocating bin/charted: _Unwind_Backtrace: symbol not found
# Error relocating bin/charted: _Unwind_GetIPInfo: symbol not found
# Error relocating bin/charted: _Unwind_GetCFA: symbol not found
# Error relocating bin/charted: _Unwind_GetTextRelBase: symbol not found
# Error relocating bin/charted: _Unwind_SetIP: symbol not found
RUN apk update && apk add --no-cache bash tini curl libgcc
WORKDIR /app/noelware/charted/server

COPY --from=build /build/target/release/charted /app/noelware/charted/server/bin/charted
COPY --from=build /build/migrations             /app/noelware/charted/server/migrations
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
