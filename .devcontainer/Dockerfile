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

ENV USERNAME=noel

# go into root, we need to install some stuff (like Docker)
USER root

ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt upgrade -y && apt install -y postgresql-client redis-tools

# go back to noel user
USER ${USERNAME}

# Add the Go toolchain here so we can develop the ClickHouse Migrations code
COPY --from=ghcr.io/auguwu/coder-images/golang /opt/golang/tools/golangci /opt/golang/tools/golangci
COPY --from=ghcr.io/auguwu/coder-images/golang /opt/golang/go             /opt/golang/go

# Add the Rust toochain so we can develop the Helm Plugin
COPY --from=ghcr.io/auguwu/coder-images/rust --chown=${USERNAME}:${USERNAME} /home/${USERNAME}/.rustup /home/${USERNAME}/.rustup
COPY --from=ghcr.io/auguwu/coder-images/rust --chown=${USERNAME}:${USERNAME} /home/${USERNAME}/.cargo  /home/${USERNAME}/.cargo

ENV LANG="en_US.UTF-8"
ENV PATH=$PATH:/opt/golang/go/bin:/opt/golang/tools/golangci:/home/${USERNAME}/.cargo/bin
