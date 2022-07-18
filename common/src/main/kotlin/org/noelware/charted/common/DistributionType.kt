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
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.common

/**
 * Represents the distribution type that the server was distributed from.
 * @param key The key to retrieve the enumeration member.
 */
@kotlinx.serialization.Serializable
enum class DistributionType(val key: String) {
    /**
     * The distribution type is unknown or was an invalid distribution type. Be cautious!
     */
    UNKNOWN("unknown"),

    /**
     * The distribution type that represents the server is running in a Docker container.
     */
    DOCKER("docker"),

    /**
     * The distribution type that represents the server was pulled from the Arch User Repository, that is maintained
     * by Noelware.
     */
    AUR("aur"),

    /**
     * The distribution type that represents the server was pulled from the Yum package manager
     * in a Fedora-based Linux distribution, maintained by Noelware.
     */
    RPM("rpm"),

    /**
     * The distribution type that represents the server was pulled from the APT package manager
     * in a Debian-based Linux distribution, maintained by Noelware.
     */
    DEB("deb"),

    /**
     * The distribution type that represents the server is running from the GitHub repository via
     * `./gradlew :server:installDist` or `make run`.
     */
    GIT("git");

    companion object {
        /**
         * Finds the distribution type via the Java system properties. The server binary will
         * implement this automatically, but it can be tampered, so be cautious!
         */
        fun fromSystemProperty(): DistributionType {
            val property = System.getProperty("org.noelware.charted.distribution.type") ?: return UNKNOWN
            return values().find { it.key == property } ?: UNKNOWN
        }
    }
}
