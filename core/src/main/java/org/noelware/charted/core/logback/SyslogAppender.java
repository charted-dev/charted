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

package org.noelware.charted.core.logback;

import ch.qos.logback.core.AppenderBase;
import ch.qos.logback.core.Layout;
import java.io.IOException;
import java.net.Socket;
import java.nio.charset.StandardCharsets;
import org.noelware.charted.common.SetOnceGetValue;

public class SyslogAppender<E> extends AppenderBase<E> {
    private final SetOnceGetValue<Socket> _socket = new SetOnceGetValue<>();

    private Layout<E> layout;
    private String host;
    private int port;

    public int getPort() {
        return port;
    }

    public Layout<E> getLayout() {
        return layout;
    }

    public String getHost() {
        return host;
    }

    public void setHost(String host) {
        this.host = host;
    }

    public void setLayout(Layout<E> layout) {
        this.layout = layout;
    }

    public void setPort(int port) {
        this.port = port;
    }

    @Override
    protected void append(E eventObject) {
        if (!_socket.wasSet()) {
            try {
                final var socket = new Socket(host, port);
                _socket.setValue(socket);
            } catch (IOException e) {
                throw new RuntimeException(e);
            }
        }

        final var socket = _socket.getValue();
        try {
            final var os = socket.getOutputStream();
            final var data = layout.doLayout(eventObject);

            os.write(data.getBytes(StandardCharsets.UTF_8));
            os.flush();
        } catch (IOException e) {
            // do nothing.
        }
    }

    @Override
    public void stop() {
        super.stop();
        if (_socket.wasSet()) {
            try {
                _socket.getValue().close();
            } catch (IOException e) {
                // ignore, we don't care
            }
        }
    }
}
