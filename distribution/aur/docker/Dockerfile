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

FROM archlinux/archlinux:base-devel-20220707.0.67222

RUN pacman -Syu --noconfirm && pacman -Sy pacman-contrib openssh git bash && useradd -ms /bin/bash noel && \
    mkdir -p /home/noel/.ssh && \
    touch /home/noel/.ssh/known_hosts

COPY ssh_config /home/noel/.ssh/config
RUN git clone https://github.com/asdf-vm/asdf.git /home/noel/.asdf && \
    cd /home/noel/.asdf && \
    git checkout -d "$(git describe --abbrev=0 --tags)" && \
    asdf asdf plugin-add java https://github.com/halcyon/asdf-java.git \
    echo "java temurin-18.0.1+10" && \
    asdf install

ENV JAVA_HOME="/home/noel/.asdf/installs/java/temurin-18.0.1+10"
ENV PATH="$PATH:/home/noel/.asdf/installs/java/temurin-18.0.1+10/bin"

# Copy the whole project (since we need Gradle to execute certain tasks)
COPY . /home/noel/projects/charted-server
RUN chmod +x /home/noel/projects/charted-server/distribution/aur/publish.sh
RUN chown noel:noel /home/noel -R
RUN chmod 600 /home/noel/.ssh/* -R

USER noel
CMD ["/home/noel/projects/charted-server/distribution/aur/publish.sh"]
