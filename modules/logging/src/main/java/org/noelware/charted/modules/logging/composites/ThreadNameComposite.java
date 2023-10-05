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

public class ThreadNameComposite extends NamedConverter {
    protected String getFullyQualifiedName(ILoggingEvent event) {
        @SuppressWarnings("StringBufferReplaceableByString")
        final StringBuilder builder = new StringBuilder();

        // bracket
        builder.append('\033').append("[38;2;134;134;134m<").append('\033').append("[0m");

        // thread name
        builder.append('\033')
                .append("[38;2;255;105;189m")
                .append(event.getThreadName())
                .append('\033')
                .append("[0m");

        // other bracket
        builder.append('\033').append("[38;2;134;134;134m>]").append('\033').append("[0m");

        return builder.toString();
    }
}