/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

package org.noelware.charted

import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.decodeFromStream
import kotlinx.serialization.json.jsonPrimitive

/**
 * Represents metadata about the API server
 */
@OptIn(ExperimentalSerializationApi::class)
public object ChartedInfo {
    /**
     * Returns the current distribution type that the server is running from.
     */
    @JvmStatic
    public val distribution: Distribution = Distribution.fromSystemProperty()

    /**
     * Returns the current version that the server is running from.
     */
    @JvmStatic
    public val version: String

    /**
     * Represents the current Git commit hash of **charted-server** that was distributed from the
     * [upstream repository](https://github.com/charted-dev/charted).
     */
    @JvmStatic
    public val commitHash: String

    /**
     * Represents the build date as an ISO-8601 represented format that was built from the source
     * in the [upstream repository](https://github.com/charted-dev/charted).
     */
    @JvmStatic
    public val buildDate: String

    /**
     * Represents the dedicated node that the server is running off. This is usually
     * only implemented in the official [distribution](https://charts.noelware.org/api/info).
     */
    @JvmStatic
    public val dedicatedNode: String? by lazy {
        val dediNode1 = System.getProperty("winterfox.dediNode", "")
        if (dediNode1.isNotEmpty()) {
            return@lazy dediNode1
        }

        val dediNode2 = System.getenv("WINTERFOX_DEDI_NODE") ?: ""
        if (dediNode2.isNotEmpty()) {
            return@lazy dediNode2
        }

        val nodeName = System.getenv("NODE_NAME") ?: ""
        if (nodeName.isNotEmpty()) {
            return@lazy nodeName
        }

        null
    }

    init {
        val stream = this::class.java.getResourceAsStream("/build-info.json") ?: error("Unable to retrieve build-info.json file from resources.")
        val data = Json.decodeFromStream<JsonObject>(stream)

        commitHash = data["commit.sha"]?.jsonPrimitive?.content ?: error("Unable to retrieve the `commit.sha` key from build-info.json file")
        buildDate = data["build.date"]?.jsonPrimitive?.content ?: error("Unable to retrieve the `build.date` key from the build-info.json file")
        version = data["version"]?.jsonPrimitive?.content ?: error("Unable to retrieve the `version` key from the build-info.json file")
    }

    /**
     * Represents the distribution that the server was distributed from.
     * @param key The key to retrieve the enumeration member.
     */
    @Serializable
    public enum class Distribution(public val key: String) {
        /**
         * Distribution type is running on a Kubernetes cluster
         */
        @SerialName("kubernetes")
        KUBERNETES("kubernetes"),

        /**
         * The distribution type is unknown or was an invalid distribution type. Be cautious!
         */
        @SerialName("unknown")
        UNKNOWN("unknown"),

        /**
         * The distribution type that represents the server is running in a Docker container.
         */
        @SerialName("docker")
        DOCKER("docker"),

        /**
         * The distribution type that represents the server was pulled from the Yum package manager
         * in a Fedora-based Linux distribution, maintained by Noelware.
         */
        @SerialName("rpm")
        RPM("rpm"),

        /**
         * The distribution type that represents the server was pulled from the APT package manager
         * in a Debian-based Linux distribution, maintained by Noelware.
         */
        @SerialName("deb")
        DEB("deb"),

        /**
         * The distribution type that represents the server is running from the GitHub repository via
         * `./gradlew :server:installDist` or `make run`.
         */
        @SerialName("git")
        GIT("git");

        override fun toString(): String = key
        public companion object {
            /**
             * Finds the distribution type via the Java system properties. The server binary will
             * implement this automatically, but it can be tampered, so be cautious!
             */
            public fun fromSystemProperty(): Distribution {
                val property = System.getProperty("org.noelware.charted.distribution.type") ?: return UNKNOWN
                return values().find { it.key == property } ?: UNKNOWN
            }
        }
    }
}
