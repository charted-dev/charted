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

package org.noelware.charted.common.extensions

import kotlinx.coroutines.runBlocking
import org.apache.commons.lang3.time.StopWatch
import org.slf4j.Logger

fun Logger.measureTime(message: String, block: () -> Unit) {
    val sw = StopWatch.createStarted()

    try {
        block()
    } catch (e: Exception) {
        sw.stop()

        val ex = Exception(e.message?.replace("%T", sw.doFormatTime()), e)
        throw ex
    }

    sw.stop()
    info(message.replace("%T", sw.doFormatTime()))
}

fun <T> Logger.measureSuspendTime(message: String, block: suspend () -> T): T {
    var result: T? = null
    measureTime(message) {
        runBlocking { result = block() }
    }

    return result!!
}
