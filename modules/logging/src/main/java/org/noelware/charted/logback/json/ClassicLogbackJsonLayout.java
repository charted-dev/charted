/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

import ch.qos.logback.classic.pattern.ThrowableHandlingConverter;
import ch.qos.logback.classic.pattern.ThrowableProxyConverter;
import ch.qos.logback.classic.spi.ILoggingEvent;
import java.text.DateFormat;
import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.LinkedHashMap;
import java.util.Map;
import java.util.TimeZone;
import org.noelware.charted.ChartedInfo;

public class ClassicLogbackJsonLayout extends LogbackJsonLayout<ILoggingEvent> {
    private final ThrowableHandlingConverter throwableProxyConverter = new ThrowableProxyConverter();

    /**
     * Transforms the given event into a {@link Map}.
     * @param event The event object that was given from {@link LogbackJsonLayout#doLayout(Object)}.
     */
    @Override
    Map<String, Object> toJsonMap(ILoggingEvent event) {
        final Map<String, Object> data = new LinkedHashMap<>();
        final DateFormat formatter = new SimpleDateFormat(getTimestampFormat());
        formatter.setTimeZone(TimeZone.getTimeZone(getTimezone()));

        data.put("@timestamp", formatter.format(new Date(event.getTimeStamp())));
        data.put("message", event.getFormattedMessage());
        data.put("thread", event.getThreadName());
        data.put("log.context", event.getLoggerContextVO().getName());
        data.put("log.level", event.getLevel().levelStr);
        data.put("log.name", event.getLoggerName());

        // === metadata ===
        final ChartedInfo info = ChartedInfo.INSTANCE;
        data.put("metadata.product", "charted-server");
        data.put("metadata.vendor", "Noelware");
        data.put("metadata.version", info.getVersion());
        data.put("metadata.commit", info.getCommitHash());
        data.put("metadata.build.date", info.getBuildDate());

        if (info.getDedicatedNode() != null) {
            data.put("metadata.dedi.node", info.getDedicatedNode());
        }

        final Map<String, String> mdc = event.getMDCPropertyMap();
        data.putAll(mdc);

        final var throwableProxy = event.getThrowableProxy();
        if (throwableProxy != null) {
            final var exception = throwableProxyConverter.convert(event);
            if (exception != null && !exception.isEmpty()) {
                data.put("exception", exception);
            }
        }

        return data;
    }

    @Override
    public void start() {
        throwableProxyConverter.start();
        super.start();
    }

    @Override
    public void stop() {
        super.stop();
        throwableProxyConverter.stop();
    }
}
