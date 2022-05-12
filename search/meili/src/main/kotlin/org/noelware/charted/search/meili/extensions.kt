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

package org.noelware.charted.search.meili

import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonNull
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.JsonPrimitive

fun ArrayList<HashMap<String, Any>>.toJsonArray(): JsonArray {
    val iter = { value: Any ->
        if (value is String)
            JsonPrimitive(value)

        if (value is Int)
            JsonPrimitive(value)

        if (value is Boolean)
            JsonPrimitive(value)

        JsonNull
    }

    return JsonArray(
        map { map ->
            JsonObject(map.toMap().mapValues(iter))
        }
    )
}
