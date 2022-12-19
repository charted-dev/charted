# 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
# Copyright 2022-2023 Noelware <team@noelware.org>
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

FROM ghcr.io/auguwu/coder-images/java

# go into root, we need to install some stuff (liek docker)
USER root

ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt upgrade -y && apt install -y postgresql-client

# go back to noel user
USER noel

# Add Go here so we can test and run Go code
COPY --from=ghcr.io/auguwu/coder-images/golang /opt/golang/tools/golangci /opt/golang/tools/golangci
COPY --from=ghcr.io/auguwu/coder-images/golang /opt/golang/go             /opt/golang/go

ENV LANG="en_US.UTF-8"
ENV PATH=$PATH:/opt/golang/go/bin:/opt/golang/tools/golangci
