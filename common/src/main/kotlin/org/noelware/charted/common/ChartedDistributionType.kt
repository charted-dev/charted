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

package org.noelware.charted.common

/**
 * Represents the distribution type the server has been distributed in.
 */
enum class ChartedDistributionType(private val key: String) {
    /**
     * The distribution type is unknown or set as an invalid distribution type.
     */
    UNKNOWN("?"),

    /**
     * The distribution type is that the server is running in a Docker container.
     */
    DOCKER("docker"),

    /**
     * The distribution type that is the server is running from the GitHub repository.
     */
    LOCAL("local"),
    RPM("rpm"),
    DEB("deb");

    companion object {
        fun fromString(): ChartedDistributionType = values().find {
            System.getProperty("org.noelware.charted.distribution.type", "") == it.key
        } ?: UNKNOWN
    }
}
