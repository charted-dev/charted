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
import org.koin.core.logger.Level
import org.koin.core.logger.Logger
import org.koin.core.logger.MESSAGE
import org.noelware.charted.common.config.Config

class KoinLogger(config: Config): Logger(if (config.debug) Level.DEBUG else Level.INFO) {
    private val log by logging<KoinLogger>()

    override fun log(level: Level, msg: MESSAGE) {
        if (this.level <= level) {
            when (level) {
                Level.DEBUG -> log.debug(msg)
                Level.ERROR -> log.error(msg)
                Level.INFO -> log.info(msg)
                Level.NONE -> {}
            }
        }
    }
}
