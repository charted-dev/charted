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

import co.elastic.apm.api.ElasticApm;
import java.io.Closeable;
import org.jetbrains.annotations.NotNull;
import org.noelware.charted.tracing.api.Span;
import org.noelware.charted.tracing.api.Transaction;
import org.noelware.charted.tracing.api.TransactionContext;
import org.noelware.charted.tracing.api.scopes.SpanScope;

public class ApmTransaction implements Transaction {
    private final co.elastic.apm.api.Transaction origin = ElasticApm.startTransaction();
    private final TransactionContext context = new TransactionContext();

    @NotNull
    @Override
    public TransactionContext getContext() {
        return context;
    }

    @NotNull
    @Override
    public Span createSpan(@NotNull String name, @NotNull String operation) {
        return null;
    }

    @Override
    public void release(@NotNull Span span) {}

    @NotNull
    @Override
    public Closeable withScope(@NotNull String name, @NotNull String operation) {
        return new SpanScope(this, createSpan(name, operation));
    }
}
