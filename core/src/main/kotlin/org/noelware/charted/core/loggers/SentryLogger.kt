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

package org.noelware.charted.core.loggers

import dev.floofy.utils.slf4j.logging
import io.sentry.ILogger
import io.sentry.Sentry
import io.sentry.SentryLevel

object SentryLogger: ILogger {
    private val log by logging<Sentry>()

    /**
     * Logs a message with the specified level, message and optional arguments.
     *
     * @param level The SentryLevel.
     * @param message The message.
     * @param args The optional arguments to format the message.
     */
    override fun log(level: SentryLevel, message: String, vararg args: Any?) {
        when (level) {
            SentryLevel.DEBUG -> log.debug(message, args)
            SentryLevel.ERROR, SentryLevel.FATAL -> log.error(message, args)
            SentryLevel.INFO -> log.info(message, args)
            SentryLevel.WARNING -> log.warn(message, args)
        }
    }

    /**
     * Logs a message with the specified level, message and optional arguments.
     *
     * @param level The SentryLevel.
     * @param message The message.
     * @param throwable The throwable to log.
     */
    override fun log(level: SentryLevel, message: String, throwable: Throwable?) {
        when (level) {
            SentryLevel.DEBUG -> log.debug(message)
            SentryLevel.ERROR, SentryLevel.FATAL -> log.error(message)
            SentryLevel.INFO -> log.info(message, throwable)
            SentryLevel.WARNING -> log.warn(message)
        }
    }

    /**
     * Logs a message with the specified level, throwable, message and optional arguments.
     *
     * @param level The SentryLevel.
     * @param throwable The throwable to log.
     * @param message The message.
     * @param args the formatting arguments
     */
    override fun log(level: SentryLevel, throwable: Throwable?, message: String, vararg args: Any?) {
        when (level) {
            SentryLevel.DEBUG -> log.debug(message, throwable, args)
            SentryLevel.ERROR, SentryLevel.FATAL -> log.error(message, throwable, args)
            SentryLevel.INFO -> log.info(message, throwable, args)
            SentryLevel.WARNING -> log.warn(message, throwable, args)
        }
    }

    /**
     * Whether this logger is enabled for the specified SentryLevel.
     *
     * @param level The SentryLevel to test against.
     * @return True if a log message would be recorded for the level. Otherwise false.
     */
    override fun isEnabled(level: SentryLevel?): Boolean =
        if ((System.getenv("org.noelware.charted.debug") ?: "false") == "true") {
            true
        } else {
            when (level) {
                SentryLevel.ERROR, SentryLevel.FATAL, SentryLevel.WARNING, SentryLevel.INFO -> true
                else -> false
            }
        }
}
