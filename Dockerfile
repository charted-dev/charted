# 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Go.
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

# This is a development container to build `charted-server` onto a local image.
# The official images uses the `release.Dockerfile` file in the root repository
# since goreleaser acts different.

FROM golang:1.18-alpine AS builder

RUN apk update && apk add --no-cache ca-certificates git make jq

WORKDIR /build/charted-server
COPY . .
RUN make deps
RUN make build

FROM alpine:3.15

WORKDIR /app/noelware/charted/server
COPY --from=builder /build/charted-server/bin .
COPY --from=builder /build/charted-server/docker /app/noelware/charted/server/scripts
COPY --from=builder /build/charted-server/assets /app/noelware/charted/server/assets

USER 1001

RUN chmod +x /app/noelware/charted/server/scripts/docker-entrypoint.sh /app/noelware/charted/server/scripts/run.sh
RUN ln -s /app/noelware/charted/server/charted-server /usr/local/bin/charted-server

ENTRYPOINT ["/app/noelware/charted/server/scripts/docker-entrypoint.sh"]
CMD ["/app/noelware/charted/server/scripts/run.sh"]
