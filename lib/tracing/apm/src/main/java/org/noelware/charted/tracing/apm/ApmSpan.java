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

import java.util.List;
import java.util.Map;
import org.jetbrains.annotations.NotNull;
import org.noelware.charted.tracing.api.Event;
import org.noelware.charted.tracing.api.Span;

public class ApmSpan implements Span {
    public ApmSpan() {}

    @NotNull
    @Override
    public String getOperation() {
        return null;
    }

    @NotNull
    @Override
    public List<Event> getEvents() {
        return null;
    }

    @NotNull
    @Override
    public String getName() {
        return null;
    }

    @NotNull
    @Override
    public Event startEvent(@NotNull String description, @NotNull Map<String, ?> metadata) {
        return null;
    }

    @Override
    public void release(@NotNull Event event) {}
}
