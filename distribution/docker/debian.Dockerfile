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

############ BINARY

FROM --platform=${TARGETPLATFORM} rust:1.78-slim-bullseye AS build

ENV DEBIAN_FRONTEND=noninteractive

RUN apt update && apt install -y libssl-dev pkg-config git ca-certificates protobuf-compiler
WORKDIR /build

COPY . .

ENV CARGO_INCREMENTAL=1
ENV RUSTFLAGS="--cfg tokio_unstable -Ctarget-cpu=native"

RUN cargo build --locked --release --bin charted

############ FINAL STAGE

FROM debian:bullseye-slim

RUN DEBIAN_FRONTEND=noninteractive apt update && apt install -y bash tini curl libssl-dev pkg-config

COPY --from=build /build/target/release/charted     /app/noelware/charted/server/bin/charted
COPY --from=build /build/migrations                 /app/noelware/charted/server/migrations
COPY distribution/docker/scripts                    /app/noelware/charted/server/scripts
COPY distribution/docker/config                     /app/noelware/charted/server/config

ENV CHARTED_DISTRIBUTION_TYPE=docker
EXPOSE 3651
VOLUME /var/lib/noelware/charted/data

RUN mkdir -p /var/lib/noelware/charted/data
RUN groupadd -g 1001 noelware && \
    useradd -rm -s /bin/bash -g noelware -u 1001 noelware &&  \
    chown noelware:noelware /app/noelware/charted/server &&   \
    chown noelware:noelware /var/lib/noelware/charted/data && \
    chmod +x /app/noelware/charted/server/scripts/docker-entrypoint.sh

# Create a symlink to `charted`
RUN ln -s /app/noelware/charted/server/bin/charted /usr/bin/charted

USER noelware
ENTRYPOINT ["/app/noelware/charted/server/scripts/docker-entrypoint.sh"]
CMD ["charted", "server"]
