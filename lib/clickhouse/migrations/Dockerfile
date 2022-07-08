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

FROM golang:1.18.3-alpine AS builder

RUN apk update
WORKDIR /build

COPY . .
RUN go build -ldflags "-s -w" -o ./bin/migrations

FROM alpine:3.16

RUN apk update && apk add tini
COPY --from=builder /build/bin/migrations ./bin/migrations

USER 1001
ENTRYPOINT ["tini", "-s"]
CMD ["/bin/migrations"]
