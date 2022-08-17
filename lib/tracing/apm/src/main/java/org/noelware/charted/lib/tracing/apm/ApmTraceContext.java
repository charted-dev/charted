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

import java.util.*;
import kotlin.Unit;
import kotlin.jvm.functions.Function1;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;
import org.noelware.charted.common.ChartedInfo;
import org.noelware.charted.lib.tracing.ISpan;
import org.noelware.charted.lib.tracing.TraceContext;
import org.noelware.charted.lib.tracing.events.EventType;

public class ApmTraceContext implements TraceContext {
    private final HashMap<String, String> attributes = new HashMap<>();
    private final ArrayList<ApmSpan> spans = new ArrayList<>();
    private final String name;

    public ApmTraceContext(String name, Map<String, String> attrs) {
        this(name);

        attributes.putAll(attrs);
    }

    public ApmTraceContext(String name) {
        this.name = name;

        attributes.putAll(
                Map.of("charted.tracing", "elastic-apm", "charted.version", ChartedInfo.INSTANCE.getVersion()));
    }

    @NotNull
    @Override
    public List<ISpan> getSpans() {
        return Collections.unmodifiableList(spans);
    }

    @NotNull
    @Override
    public Map<String, String> getAttributes() {
        return Collections.unmodifiableMap(attributes);
    }

    @NotNull
    @Override
    public String getName() {
        return name;
    }

    @NotNull
    @Override
    public ISpan startSpan(@NotNull String name, @NotNull String operation) {
        var span = new ApmSpan(name, operation);
        spans.add(span);

        return span;
    }

    @Nullable
    @Override
    public ISpan stopSpan(@NotNull String name) {
        var span =
                spans.stream().filter(it -> it.getName().equalsIgnoreCase(name)).findAny();

        if (span.isEmpty()) return null;

        var s = span.get();
        spans.remove(s);

        return s;
    }

    @NotNull
    @Override
    public ISpan withScope(
            @NotNull String name, @NotNull String operation, @NotNull Function1<? super ISpan, Unit> block) {
        var span = startSpan(name, operation);
        var event = span.startEvent("used #withScope(%s, %s)".formatted(name, operation), Map.of());

        try {
            block.invoke(span);
            event.setType(EventType.SuccessEventType);
        } catch (Exception e) {
            event.set("exception", e);
            event.setType(EventType.ExceptionEventType);
        } finally {
            stopSpan(span.getName());
        }

        return span;
    }
}
