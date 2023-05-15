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

import co.elastic.apm.api.ElasticApm;
import co.elastic.apm.api.Scope;
import io.ktor.server.application.Application;
import java.util.Objects;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;
import org.noelware.charted.modules.tracing.Span;
import org.noelware.charted.modules.tracing.Tracer;
import org.noelware.charted.modules.tracing.Transaction;

public class APMTransaction implements Transaction {
    private final co.elastic.apm.api.Transaction inner;
    private final String operation;
    private final Tracer parent;
    private final Scope scope;
    private final String name;

    public APMTransaction(@NotNull String name, @Nullable String operation, @NotNull Tracer tracer) {
        this.operation = operation;
        this.parent = Objects.requireNonNull(tracer, "Parent tracer cannot be null");
        this.inner = ElasticApm.startTransaction();
        this.name = Objects.requireNonNull(name, "Transaction name cannot be null");

        inner.setName(name);
        inner.setFrameworkName(
                "Ktor %s".formatted(Application.class.getPackage().getImplementationVersion()));

        inner.setType(operation != null ? operation : co.elastic.apm.api.Transaction.TYPE_REQUEST);

        this.scope = inner.activate();
    }

    @Override
    @NotNull
    public Span createSpan(@NotNull String name, @Nullable String operation) {
        return new APMSpan(name, operation, this, inner.startSpan(name, operation, "<unknown>"));
    }

    @Override
    @NotNull
    public Span createSpan(@NotNull String name) {
        return createSpan(name, null);
    }

    @Override
    @NotNull
    public Tracer tracer() {
        return parent;
    }

    @Override
    @Nullable
    public String operation() {
        return operation;
    }

    @Override
    @NotNull
    public String name() {
        return name;
    }

    @Override
    public void end(@Nullable Throwable throwable) {
        if (throwable != null) {
            inner.captureException(throwable);
        }

        scope.close();
        inner.end();
    }
}
