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

package org.noelware.charted.modules.tracing.elastic;

import java.util.Objects;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;
import org.noelware.charted.modules.tracing.Span;
import org.noelware.charted.modules.tracing.Transaction;

/**
 * Represents a {@link Span} that backs up a {@link co.elastic.apm.api.Span APM span}.
 */
public class APMSpan implements Span {
    private final co.elastic.apm.api.Span inner;
    private final Transaction parent;
    private final String operation;
    private final String name;

    public APMSpan(String name, String operation, Transaction parent, co.elastic.apm.api.Span inner) {
        this.operation = operation;
        this.parent = Objects.requireNonNull(parent, "Parent transaction cannot be null");
        this.inner = Objects.requireNonNull(inner, "Inner APM span cannot be null");
        this.name = Objects.requireNonNull(name, "Span name cannot be null");
    }

    @NotNull
    @Override
    public Transaction transaction() {
        return parent;
    }

    @Nullable
    @Override
    public String operation() {
        return operation;
    }

    @NotNull
    @Override
    public String name() {
        return name;
    }

    @Override
    public void end(@Nullable Throwable throwable) {
        if (throwable != null) {
            inner.captureException(throwable);
        }

        inner.end();
    }
}
