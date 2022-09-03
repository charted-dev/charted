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

# Maintainer: Noelware Team <team@noelware.org>
# Contributor: Noel <cutie@floofy.dev>

pkgname=charted-server
pkgver="${version}"
pkgrel=1
pkgdesc="📦 Free, open source, and reliable Helm Chart registry made in Kotlin"
arch=("x86_64")
url="https://charts.noelware.org"
license=('Apache-2.0')
depends=('jdk17-openjdk')
source=('charted-server.tar.gz::https://dl.noelware.org/charted/server/${arch}/${pkgver}.tar.gz')
sha256sums=('${checksum}')

package() {
    install -Dm644 "${srcdir}"/LICENSE "${pkgdir}"/usr/share/licenses/charted-server/LICENSE
    install -Dm755 "${srcdir}"/charted.service "${pkgdir}"/usr/lib/systemd/system/charted.service

    # main installation will live in, the data will
    # be stored in /var/lib/noelware/charted-server
    mkdir -p "${pkgdir}"/etc/noelware/charted/server
    mkdir -p "${pkgdir}"/etc/noelware/charted/server/lib
    mkdir -p "${pkgdir}"/etc/noelware/charted/server/config
    mkdir -p "${pkgdir}"/etc/noelware/charted/server/bin
    mkdir -p "${pkgdir}"/var/lib/noelware/charted-server

    cp -r "${srcdir}"/lib "${pkgdir}"/etc/noelware/charted/server/lib
    install -Dm644 "${srcdir}"/charted.example.yml "${pkgdir}"/etc/noelware/charted/server/config/charted.yaml
    install -Dm644 "${srcdir}"/logback.properties "${pkgdir}"/etc/noelware/charted/server/config/logback.properties
    install -Dm755 "${srcdir}"/charted-server "${pkgdir}"/etc/noelware/charted/server/bin/charted-server
}
