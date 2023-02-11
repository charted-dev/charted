/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.decodeFromStream
import kotlinx.serialization.json.jsonPrimitive

@OptIn(ExperimentalSerializationApi::class)
public object ChartedInfo {
    /**
     * Returns the current distribution type that the server is running from.
     */
    @JvmStatic
    public val distribution: DistributionType = DistributionType.fromSystemProperty()

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
}
