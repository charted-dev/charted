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

import ch.qos.logback.core.AppenderBase
import ch.qos.logback.core.Layout
import okhttp3.internal.closeQuietly
import org.noelware.charted.common.SetOnceGetValue
import java.net.Socket
import kotlin.properties.Delegates

class SyslogAppender<E>: AppenderBase<E>() {
    private val _socket: SetOnceGetValue<Socket> = SetOnceGetValue()

    lateinit var layout: Layout<E>
    lateinit var host: String
    var port by Delegates.notNull<Int>()

    override fun append(event: E) {
        if (!_socket.wasSet()) {
            val socket = Socket(host, port)
            _socket.value = socket
        }

        val socket = _socket.value
        val os = socket.getOutputStream()
        val data = layout.doLayout(event)

        os.write(data.toByteArray())
        os.flush()
    }

    override fun stop() {
        super.stop()
        if (_socket.wasSet()) {
            _socket.value.closeQuietly()
        }
    }
}
