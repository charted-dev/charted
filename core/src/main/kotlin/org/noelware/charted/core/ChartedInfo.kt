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

package org.noelware.charted.core

import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.decodeFromStream
import kotlinx.serialization.json.jsonPrimitive

@OptIn(ExperimentalSerializationApi::class)
object ChartedInfo {
    /**
     * Represents the current version of **charted-server** from the `build-info.json` file
     * that is present in resources.
     */
    val version: String

    /**
     * Represents the current git commit hash of **charted-server** from the `build-info.json` file
     * that is present in resources.
     */
    val commitHash: String

    /**
     * Represents the build date of when **charted-server** was built from source in the `build-info.json` file
     * that is present in resources.
     */
    val buildDate: String

    /**
     * Returns the distribution type.
     */
    val distributionType: DistributionType = DistributionType.fromString(System.getProperty("org.noelware.charted.distribution.type", ""))

    /**
     * Returns the dedicated node the server is running off. This is usually
     * present in `cdn.floofy.dev` or `cdn.noelware.org`
     */
    val dediNode by lazy {
        // Check if we have `winterfox.dediNode` in the Java properties
        val dediNode1 = System.getProperty("winterfox.dediNode", "")
        if (dediNode1.isNotEmpty()) {
            return@lazy dediNode1
        }

        // Maybe we only have the `WINTERFOX_DEDI_NODE` environment variable?
        // If we do, we'll assume that it is the dedi node name!
        val dediNode2 = System.getenv("WINTERFOX_DEDI_NODE")
        if (dediNode2 != null) {
            return@lazy dediNode2
        }

        // We can't find anything :(
        null
    }

    init {
        val stream = this::class.java.getResourceAsStream("/build-info.json")!!
        val data = Json.decodeFromStream<JsonObject>(stream)

        version = data["version"]?.jsonPrimitive?.content ?: error("Unable to retrieve `version` from build-info.json!")
        commitHash = data["commit.sha"]?.jsonPrimitive?.content ?: error("Unable to retrieve `commit.sha` from build-info.json!")
        buildDate = data["build.date"]?.jsonPrimitive?.content ?: error("Unable to retrieve `build.date` from build-info.json!")
    }
}
