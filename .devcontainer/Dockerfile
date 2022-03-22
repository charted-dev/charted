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

FROM auguwu/devcontainers:latest

ARG GO_VERSION="1.18"
RUN mkdir -p /opt/golang
RUN curl -L -o /tmp/golang.tar.gz https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz
RUN tar -xvf /tmp/golang.tar.gz -C /opt/golang
RUN rm /tmp/golang.tar.gz
ENV PATH="/opt/golang/go${GO_VERSION}.linux-amd64/bin:$PATH"

CMD ["sleep", "infinity"]
