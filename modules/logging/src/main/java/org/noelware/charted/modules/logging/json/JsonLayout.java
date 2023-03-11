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

package org.noelware.charted.modules.logging.json;

import ch.qos.logback.core.LayoutBase;
import java.time.ZoneId;
import java.util.Map;
import org.noelware.charted.modules.logging.json.formatters.DefaultJsonFormatter;
import org.noelware.charted.modules.logging.json.formatters.JsonFormatter;

public abstract class JsonLayout<E> extends LayoutBase<E> {
    private final JsonFormatter formatter = new DefaultJsonFormatter();
    private String timestampFormat = "yyyy-MM-dd'T'HH:mm:ssXXX";
    private ZoneId timezone = ZoneId.systemDefault();

    /**
     * Transforms the given event into a {@link Map}.
     * @param event The event object that was given from {@link #doLayout(E)}.
     */
    abstract Map<String, Object> toJsonMap(E event);

    public void setPrettyPrint(String value) {
        setPrettyPrint(Boolean.parseBoolean(value));
    }

    /**
     * Sets the default JSON formatter's pretty printing status.
     * @param value boolean.
     */
    public void setPrettyPrint(boolean value) {
        ((DefaultJsonFormatter) formatter).setIsPrettyPrint(value);
    }

    public String getTimestampFormat() {
        return timestampFormat;
    }

    public void setTimestampFormat(String timezoneFormat) {
        this.timestampFormat = timezoneFormat;
    }

    public ZoneId getTimezone() {
        return timezone;
    }

    public void setTimezone(ZoneId timezone) {
        this.timezone = timezone;
    }

    @Override
    public String doLayout(E event) {
        final Map<String, Object> data = toJsonMap(event);
        if (data == null || data.isEmpty()) return null;

        try {
            return formatter.doFormat(data);
        } catch (Exception e) {
            addError("Received error while transforming to JSON", e);
            return data.toString();
        }
    }
}
