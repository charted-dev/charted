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

package org.noelware.charted.modules.logging.composites;

import ch.qos.logback.classic.pattern.NamedConverter;
import ch.qos.logback.classic.spi.ILoggingEvent;
import java.util.Map;

public class MdcComposite extends NamedConverter {
    @Override
    protected String getFullyQualifiedName(ILoggingEvent event) {
        final Map<String, String> mdc = event.getMDCPropertyMap();
        if (mdc.isEmpty()) return "";

        final StringBuilder builder = new StringBuilder();
        int idx = 0;

        for (Map.Entry<String, String> entry : mdc.entrySet()) {
            // More than 15 entries is probably too crazy to log!
            if (idx++ == 15) break;

            builder.append('[')
                    .append(entry.getKey())
                    .append(": ")
                    .append(entry.getValue())
                    .append(']')
                    .append(' ');
        }

        return builder.toString();
    }
}
