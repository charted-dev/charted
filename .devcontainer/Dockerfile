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

FROM mcr.microsoft.com/vscode/devcontainers/base:bullseye

ARG USERNAME=vscode
ARG UID=1000
ARG GID=$UID

ENV DEBIAN_FRONTEND=noninteractive
RUN apt upgrade && apt update && \
    apt install -y sudo && \
    echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME && \
    apt install -y --no-install-recommends zip unzip bash bash-completion build-essential \
        jq locales man-db software-properties-common time lsof openssh-client git curl

# Install Java 18
ENV JAVA_INSTALL="https://github.com/adoptium/temurin18-binaries/releases/download/jdk-18.0.1%2B10/OpenJDK18U-jdk_x64_linux_hotspot_18.0.1_10.tar.gz"
RUN curl -L -o /tmp/java.tar.gz $JAVA_INSTALL && \
    mkdir -p /opt/java && \
    tar -xvf /tmp/java.tar.gz -C /opt/java && \
    rm /tmp/java.tar.gz

ENV JAVA_HOME="/opt/java/jdk-18.0.1+10"
ENV PATH="$PATH:/opt/java/jdk-18.0.1+10/bin"

# Install Gradle 7.5.1
ENV GRADLE_INSTALL="https://services.gradle.org/distributions/gradle-7.5.1-all.zip"
RUN curl -L -o /tmp/gradle.zip $GRADLE_INSTALL && \
    mkdir -p /opt/gradle && \
    unzip -d /opt/gradle /tmp/gradle.zip && \
    rm /tmp/gradle.zip

ENV PATH="$PATH:/opt/gradle/gradle-7.5/bin"
RUN apt autoremove -y && \
    apt clean -y && \
    rm -rf /var/lib/apt/lists/*

ENV DEBIAN_FRONTEND=dialog
USER $USERNAME
