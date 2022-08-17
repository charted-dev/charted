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

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Map;
import org.jetbrains.annotations.NotNull;
import org.noelware.charted.lib.tracing.IEvent;
import org.noelware.charted.lib.tracing.ISpan;
import org.noelware.charted.lib.tracing.events.EventType;

public class ApmSpan implements ISpan {
    private final ArrayList<ApmEvent> events = new ArrayList<>();
    private final String operation;
    private final String name;

    public ApmSpan(String name, String operation) {
        this.operation = operation;
        this.name = name;
    }

    @NotNull
    @Override
    public String getOperation() {
        return operation;
    }

    @NotNull
    @Override
    public List<IEvent> getEvents() {
        return Collections.unmodifiableList(events);
    }

    @NotNull
    @Override
    public String getName() {
        return name;
    }

    @NotNull
    @Override
    public IEvent startEvent(@NotNull String description, @NotNull Map<String, ?> metadata) {
        var event = new ApmEvent(description, EventType.EmptyEventType);
        events.add(event);

        return event;
    }
}
