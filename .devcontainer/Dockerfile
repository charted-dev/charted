# 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

FROM buildpack-deps:focal-scm

ARG USERNAME=vscode
ARG UID=1000
ARG GID=$UID

ENV DEBIAN_FRONTEND=noninteractive
RUN set -ex; \
    apt-get upgrade && apt-get update; \
    groupadd --gid $GID $USERNAME; \
    useradd -s /bin/bash --uid $UID --gid $GID -m $USERNAME; \
    apt-get install -y sudo; \
    echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME; \
    apt-get install -y --no-install-recommends \
        zip \
        unzip \
        bash-completion \
        build-essential \
        jq \
        locales \
        man-db \
        software-properties-common \
        time \
        vim \
        lsof \
        openssh-client iproute2 lsb-release git; \
    git clone https://github.com/asdf-vm/asdf.git ~/.asdf --branch v0.10.0; \
    echo ". \$HOME/.asdf/asdf.sh" >> ~/.bashrc; \
    echo ". \$HOME/.asdf/completions/asdf.bash" >> ~/.bashrc; \
    source ~/.bashrc; \
    asdf plugin add nodejs https://github.com/asdf-vm/asdf-nodejs.git; \
    asdf plugin add java https://github.com/halcyon/asdf-java.git; \
    asdf plugin add gradle https://github.com/rfrancis/asdf-gradle.git; \
    echo "nodejs 18.0.0" >> ~/.tool-versions; \
    echo "gradle 7.5" >> ~/.tool-versions; \
    echo "java   temurin-18.0.1+10" >> ~/.tool-versions; \
    asdf install; \
    apt-get autoremove -y; \
    apt-get clean -y; \
    rm -rf /var/lib/apt/lists/*

ENV DEBIAN_FRONTEND=dialog
