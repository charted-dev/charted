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

package org.noelware.charted.modules.logging.json.formatters;

import com.fasterxml.jackson.core.JsonGenerator;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.io.StringWriter;
import java.io.Writer;
import java.util.Map;

public class DefaultJsonFormatter implements JsonFormatter {
    private final ObjectMapper MAPPER = new ObjectMapper();
    private boolean isPrettyPrint = false;

    /**
     * Returns whether pretty printing should be enabled. Enabled by default,
     * can be disabled with the <code>charted.json.pretty-print</code> configuration
     * key.
     */
    public boolean isPrettyPrintEnabled() {
        return isPrettyPrint;
    }

    /**
     * Sets the value for whether pretty printing should be enabled.
     * @param value The value.
     */
    public void setIsPrettyPrint(boolean value) {
        this.isPrettyPrint = value;
    }

    @Override
    public String doFormat(Map<String, Object> data) throws Exception {
        final Writer writer = new StringWriter(512);
        final JsonGenerator generator = MAPPER.getFactory().createGenerator(writer);

        if (isPrettyPrintEnabled()) {
            generator.useDefaultPrettyPrinter();
        }

        MAPPER.writeValue(writer, data);
        writer.flush();

        return writer.toString() + "\n";
    }
}
