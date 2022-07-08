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

FROM alpine:3.16

RUN apk update && apk add --no-cache curl libc6-compat gcompat bash
RUN curl -L -o /usr/local/bin/clickhouse-client https://builds.clickhouse.tech/master/amd64/clickhouse
RUN chmod +x /usr/local/bin/clickhouse-client
COPY bin/*.sh /usr/local/bin

ENTRYPOINT ["docker-entrypoint.sh"]
