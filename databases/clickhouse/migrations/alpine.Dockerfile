# 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
# Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

FROM --platform=$BUILDPLATFORM golang:1.20-alpine3.17 AS builder

ARG VERSION

WORKDIR /build
COPY . .
RUN go get && go build -mod=readonly -ldflags "-s -w -X main.version=${VERSION}" -o ./bin/ch-migrations

FROM --platform=$BUILDPLATFORM alpine:3.17

RUN apk update && apk add --no-cache tini bash
WORKDIR /app/noelware/charted/migrations

COPY --from=builder /build/bin/ch-migrations /app/noelware/charted/migrations/bin/ch-migrations

RUN addgroup -g 1001 noelware && \
    adduser -DSH -u 1001 -G noelware noelware && \
    chown -R noelware:noelware /app/noelware/charted/migrations

USER noelware
ENTRYPOINT ["tini", "-s"]
CMD ["/app/noelware/charted/migrations/bin/ch-migrations"]
