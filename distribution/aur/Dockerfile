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

FROM archlinux/archlinux:base

RUN pacman -Syyu --no-confirm && \
    pacman -Syyu --no-confirm pacman-contrib openssh git fakeroot \
    binutils go-pie gcc awk binutils xz libarchive bzip2 coreutils file \
    findutils gettext grep gzip sed ncurses

RUN useradd -ms /bin/bash github && \
    mkdir -p /home/github/.ssh && \
    touch /home/github/.ssh/known_hosts

COPY ssh_config /home/github/.ssh/config
RUN chown github:github /home/github -R && \
    chown 600 /home/github/.ssh/* -R

COPY publish.sh .
COPY build/generated/aur/PKGBUILD .

USER github
WORKDIR /home/github

ENTRYPOINT ["/publish.sh"]
