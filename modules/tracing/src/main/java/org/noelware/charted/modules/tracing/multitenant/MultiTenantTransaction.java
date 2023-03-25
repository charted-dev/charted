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

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;
import org.noelware.charted.modules.tracing.Span;
import org.noelware.charted.modules.tracing.Tracer;
import org.noelware.charted.modules.tracing.Transaction;

public class MultiTenantTransaction implements Transaction {
    private final List<Transaction> transactions;
    private final Tracer parent;

    public MultiTenantTransaction(Tracer parent, List<Transaction> transactions) {
        this.transactions = transactions;
        this.parent = Objects.requireNonNull(parent);
    }

    @Override
    public @NotNull Span createSpan(@NotNull String name, @Nullable String operation) {
        final ArrayList<Span> spans = new ArrayList<>();
        for (Transaction transaction : transactions) spans.add(transaction.createSpan(name, operation));

        return new MultiTenantSpan(this, spans);
    }

    @Override
    public @NotNull Span createSpan(@NotNull String name) {
        return createSpan(name, null);
    }

    @Override
    public @NotNull Tracer tracer() {
        return parent;
    }

    @Override
    public @Nullable String operation() {
        if (transactions.size() == 1) {
            return transactions.get(0).operation();
        }

        final StringBuilder builder = new StringBuilder();
        for (Transaction transaction : transactions) {
            builder.append("transaction ").append(transaction.name());

            final String op = transaction.operation();
            if (op != null) builder.append(": ").append(op);

            builder.append('\n');
        }

        return builder.toString().trim();
    }

    @Override
    public @NotNull String name() {
        if (transactions.size() == 1) {
            return transactions.get(0).name();
        }

        final StringBuilder builder = new StringBuilder();
        for (Transaction transaction : transactions) {
            builder.append("transaction ")
                    .append(transaction.name())
                    .append(" (")
                    .append(transaction)
                    .append(')')
                    .append('\n');
        }

        return builder.toString().trim();
    }

    @Override
    public void close() throws Exception {
        for (Transaction transaction : transactions) transaction.close();
    }
}
