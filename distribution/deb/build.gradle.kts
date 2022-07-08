/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022 Noelware <team@noelware.org>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import com.netflix.gradle.plugins.deb.Deb

plugins {
    id("nebula.ospackage-base")
}

ospackage {
    maintainer = "Noelware, LCC. <team@noelware.org>"
    summary = "\uD83D\uDCE6 Free, open source, and reliable Helm Chart registry made in Kotlin"
    url = "https://charts.noelware.org"

    packageDescription = """
    |charted-server is the backend server for the charted Project by Noelware. It serves to be
    |a reliable, free, and open sourced Helm Chart registry server to reliable distribute Helm Charts
    |with a minimal configuration, yet a highly-customizable experience!
    |
    |This is a RESTful API, if you wish to have a frontend installed, you can install Pak, which is
    |the web dashboard for charted.
    |   ❯ https://charts.noelware.org/docs/frontend
    |   
    |Want more information? You can read up on our documentation:
    |   ❯ https://charts.noelware.org/docs
    |   
    |Want to demo a instance, or view the official one ran by Noelware?
    |   ❯ https://charts.noelware.org/demo
    |   ❯ https://charts.noelware.org
    |   
    |Any issues occur while running the server? You can report it to the charted team via
    |GitHub issues:
    |   ❯ https://github.com/charted-dev/charted/issues
    |   
    |~ Noelware, LLC. ^-^ ヾ(*ΦωΦ)ﾉ
    """.trimMargin()

    if (System.getenv("NOELWARE_SIGNING_PASSWORD") != null) {
        signingKeyPassphrase = System.getenv("NOELWARE_SIGNING_PASSWORD")!!
        signingKeyId = System.getenv("NOELWARE_SIGNING_ID")!!

        val ringPath = System.getenv("NOELWARE_SIGNING_RING_PATH")
        signingKeyRingFile = if (ringPath != null) file(ringPath) else
            File(File(System.getProperty("user.home")), ".gnupg/secring.gpg")
    }

    permissionGroup = "root"
    fileMode = "0644".toInt()
    dirMode = "0755".toInt()
    user = "root"

    into("/etc/noelware/charted/server")
}

tasks.register<Deb>("installDeb") {
    packageDescription = "\uD83D\uDCE6 Free, open source, and reliable Helm Chart registry made in Kotlin"
    release = "1"
    vendor = "Noelware, LLC. <team@noelware.org>"
}
