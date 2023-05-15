/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

package org.noelware.charted.modules.tracing.multitenant;

import java.util.List;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;
import org.noelware.charted.modules.tracing.Span;
import org.noelware.charted.modules.tracing.Transaction;

/**
 * Represents a wrapper for {@link Span spans} to be collected from more
 * than once tracer.
 */
public class MultiTenantSpan implements Span {
    private final Transaction parent;
    private final List<Span> spans;

    /**
     * Constructs a new {@link MultiTenantSpan}.
     * @param parent The parent transaction that this span belongs to. This will most
     *               likely be a {@link MultiTenantTransaction}.
     *
     * @param spans A list of the spans that this {@link MultiTenantSpan} takes
     *              care of.
     */
    public MultiTenantSpan(Transaction parent, List<Span> spans) {
        this.parent = parent;
        this.spans = spans;
    }

    @Override
    public @NotNull Transaction transaction() {
        return parent;
    }

    @Override
    public @Nullable String operation() {
        if (spans.size() == 1) {
            return spans.get(0).operation();
        }

        final StringBuilder builder = new StringBuilder();
        for (Span span : spans) {
            builder.append("span ")
                    .append(span.name())
                    .append(" in transaction ")
                    .append('[')
                    .append(parent.toString())
                    .append(']');

            final String op = span.operation();
            if (op != null) builder.append(": ").append(op);

            builder.append('\n');
        }

        return builder.toString().trim();
    }

    @Override
    public @NotNull String name() {
        if (spans.size() == 1) {
            return spans.get(0).name();
        }

        final StringBuilder builder = new StringBuilder();
        for (Span span : spans) {
            builder.append("span ")
                    .append(span.name())
                    .append(" in transaction ")
                    .append('[')
                    .append(parent.toString())
                    .append(']')
                    .append('\n');
        }

        return builder.toString().trim();
    }

    public void end(@Nullable Throwable throwable) {
        for (Span span : spans) span.end(throwable);
    }
}
