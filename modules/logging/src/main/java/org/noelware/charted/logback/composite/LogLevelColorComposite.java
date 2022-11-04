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

package org.noelware.charted.logback.composite;

import ch.qos.logback.classic.Level;
import ch.qos.logback.classic.spi.ILoggingEvent;
import ch.qos.logback.core.pattern.color.ANSIConstants;
import ch.qos.logback.core.pattern.color.ForegroundCompositeConverterBase;

public class LogLevelColorComposite extends ForegroundCompositeConverterBase<ILoggingEvent> {
    @Override
    protected String getForegroundColorCode(ILoggingEvent event) {
        final var level = event.getLevel();
        return switch (level.toInt()) {
            case Level.ERROR_INT -> ANSIConstants.BOLD + "38;2;166;76;76"; // red
            case Level.WARN_INT -> ANSIConstants.BOLD + "38;2;234;234;208"; // yellow
            case Level.INFO_INT -> ANSIConstants.BOLD + "38;2;81;81;140"; // bluuu
            case Level.DEBUG_INT -> ANSIConstants.BOLD + "38;2;241;204;209"; // pink
            case Level.TRACE_INT -> ANSIConstants.BOLD + "38;2;156;156;252"; // purple
            default -> ANSIConstants.DEFAULT_FG;
        };
    }
}
