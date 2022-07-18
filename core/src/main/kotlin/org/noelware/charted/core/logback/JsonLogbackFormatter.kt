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

package org.noelware.charted.core.logback

import ch.qos.logback.contrib.json.JsonFormatter
import com.fasterxml.jackson.databind.ObjectMapper
import java.io.StringWriter

class JsonLogbackFormatter: JsonFormatter {
    private val mapper: ObjectMapper = ObjectMapper()

    override fun toJsonString(m: MutableMap<Any?, Any?>?): String {
        val writer = StringWriter(512)
        val generator = mapper.factory.createGenerator(writer)

        mapper.writeValue(generator, m)
        writer.flush()

        return writer.toString() + "\n"
    }
}
