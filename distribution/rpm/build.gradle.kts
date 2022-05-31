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

import com.netflix.gradle.plugins.rpm.Rpm

plugins {
    `charted-distribution-module`
    id("nebula.ospackage-base")
}

ospackage {
    maintainer = "Noelware, LLC. <team@noelware.org>"
    summary = "\uD83D\uDCE6 Free, open source, and reliable Helm Chart registry made in Kotlin"
    url = "https://charts.noelware.org"

    packageDescription = """
    |This is a Helm Chart registry to reliably distribute Helm Charts without configuring anything!
    |
    |For more information, you can read on our documentation:
    |   https://charts.noelware.org/docs
    |
    |Interested to see how charted-server was built? You can check on GitHub (and star it if you like it!):
    |   https://github.com/charted-dev/charted
    |
    |Seen any issues while running charted-server? You can always report it in GitHub Issues:
    |   https://github.com/charted-dev/charted/issues
    |
    |~ Noelware ヾ(*ΦωΦ)ﾉ
    """.trimMargin()

    if (System.getenv("NOELWARE_SIGNING_PASSWORD") != null) {
        signingKeyPassphrase = System.getenv("NOELWARE_SIGNING_PASSWORD")!!
        signingKeyId = System.getenv("NOELWARE_SIGNING_ID") ?: "0000000"

        val ringFilePath = System.getenv("NOELWARE_SIGNING_RING_FILE_PATH")
        signingKeyRingFile = if (ringFilePath != null) {
            file(ringFilePath)
        } else {
            File(File(System.getProperty("user.home") ?: ".gnupg"), "secring.gpg")
        }
    }

    permissionGroup = "root"
    fileMode = "0644".toInt()
    dirMode = "0755".toInt()
    user = "root"

    into("/etc/noelware/charted")
}

tasks.register<Rpm>("rpm") {
    packageDescription = "\uD83D\uDCE6 Free, open source, and reliable Helm Chart registry made in Kotlin"
    release = "1"
    vendor = "Noelware, LLC. <team@noelware.org>"
}
