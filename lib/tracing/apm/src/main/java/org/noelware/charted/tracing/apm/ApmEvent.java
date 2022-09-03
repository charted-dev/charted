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

package org.noelware.charted.tracing.apm;

import java.util.Collections;
import java.util.HashMap;
import java.util.Map;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;
import org.noelware.charted.tracing.api.Clock;
import org.noelware.charted.tracing.api.Event;

public class ApmEvent implements Event {
    private final HashMap<String, Object> metadata = new HashMap<>();
    private final String description;

    private long resultTime = -1;

    public ApmEvent(String description) {
        this.description = description;
    }

    @NotNull
    @Override
    public String getDescription() {
        return description;
    }

    @NotNull
    @Override
    public Map<String, Object> getMetadata() {
        return Collections.unmodifiableMap(metadata);
    }

    @Override
    public long getResultTime() {
        return resultTime;
    }

    @Override
    public <T> void set(@NotNull String key, @NotNull T value) {
        metadata.putIfAbsent(key, value);
    }

    @Nullable
    @Override
    public <T> T get(@NotNull String key) {
        final var result = metadata.get(key);
        if (result == null) return null;

        return (T) result;
    }

    /**
     * Closes this stream and releases any system resources associated
     * with it. If the stream is already closed then invoking this
     * method has no effect.
     *
     * <p> As noted in {@link AutoCloseable#close()}, cases where the
     * close may fail require careful attention. It is strongly advised
     * to relinquish the underlying resources and to internally
     * <em>mark</em> the {@code Closeable} as closed, prior to throwing
     * the {@code IOException}.
     *
     */
    @Override
    public void close() {
        this.resultTime = Clock.NanoTime.INSTANCE.provide();
    }
}
