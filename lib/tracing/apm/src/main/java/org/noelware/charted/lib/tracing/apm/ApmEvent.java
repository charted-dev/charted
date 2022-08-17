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

package org.noelware.charted.lib.tracing.apm;

import java.util.Collections;
import java.util.HashMap;
import java.util.Map;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;
import org.noelware.charted.lib.tracing.IEvent;
import org.noelware.charted.lib.tracing.events.EventType;

public class ApmEvent implements IEvent {
    private final HashMap<String, Object> metadata = new HashMap<>();
    private final String description;
    private EventType eventType;

    public ApmEvent(String description, EventType type) {
        this.eventType = type;
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

    @NotNull
    @Override
    public EventType getType() {
        return eventType;
    }

    @Override
    public void setType(@NotNull EventType value) {
        this.eventType = value;
    }

    @Override
    public <T> void set(@NotNull String key, @NotNull T value) {
        metadata.putIfAbsent(key, value);
    }

    @SuppressWarnings("unchecked")
    @Nullable
    @Override
    public <T> T get(@NotNull String key) {
        return (T) metadata.getOrDefault(key, null);
    }
}
