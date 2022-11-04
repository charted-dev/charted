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

package org.noelware.charted.logback.json;

import ch.qos.logback.core.LayoutBase;
import java.util.Map;
import org.noelware.charted.logback.json.internal.DefaultLogbackJsonFormatter;

public abstract class LogbackJsonLayout<E> extends LayoutBase<E> {
    private LogbackJsonFormatter FORMATTER = new DefaultLogbackJsonFormatter();

    /**
     * Returns the given {@link LogbackJsonFormatter} for formatting Java maps
     * into JSON strings.
     */
    public LogbackJsonFormatter getFormatter() {
        return FORMATTER;
    }

    /**
     * Sets the formatter to use in this {@link LogbackJsonLayout}.
     * @param formatter The formatter to use.
     */
    public void setFormatter(LogbackJsonFormatter formatter) {
        FORMATTER = formatter;
    }

    /**
     * Sets the default JSON formatter's pretty printing status.
     * @param value boolean.
     */
    public void setIsPrettyPrinting(boolean value) {
        ((DefaultLogbackJsonFormatter) FORMATTER).setUsePrettyPrint(value);
    }

    /**
     * Transforms the given event into a {@link Map}.
     * @param event The event object that was given from {@link #doLayout(E)}.
     */
    abstract Map<String, Object> toJsonMap(E event);

    /**
     * Transform an event (of type Object) and return it as a String after
     * appropriate formatting.
     *
     * <p>
     * Taking in an object and returning a String is the least sophisticated way of
     * formatting events. However, it is remarkably CPU-effective.
     * </p>
     *
     * @param event The event to format
     * @return the event formatted as a String
     */
    @Override
    public String doLayout(E event) {
        final Map<String, Object> map = toJsonMap(event);
        if (map == null || map.isEmpty()) return null;

        try {
            return FORMATTER.doFormat(map);
        } catch (Exception e) {
            addError("Received error while transforming JSON data, defaulting to Map#toString", e);
            return map.toString();
        }
    }
}
