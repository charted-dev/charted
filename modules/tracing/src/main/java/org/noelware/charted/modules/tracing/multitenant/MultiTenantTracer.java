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

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;
import org.noelware.charted.modules.tracing.Tracer;
import org.noelware.charted.modules.tracing.Transaction;

/**
 * Represents a wrapper for including multiple tracers at once, if configured.
 */
public class MultiTenantTracer implements Tracer {
    private final List<Tracer> tracers;

    /**
     * Creates a new {@link MultiTenantTracer}.
     * @param tracers A list of all configured tracers
     */
    public MultiTenantTracer(List<Tracer> tracers) {
        this.tracers = tracers;
    }

    @Override
    public @NotNull Transaction createTransaction(@NotNull String name, @Nullable String operation) {
        final ArrayList<Transaction> transactions = new ArrayList<>();
        for (Tracer tracer : tracers) transactions.add(tracer.createTransaction(name, operation));

        return new MultiTenantTransaction(this, transactions);
    }

    @Override
    public @NotNull Transaction createTransaction(@NotNull String name) {
        return createTransaction(name, null);
    }

    @Override
    public void close() throws IOException {
        for (Tracer tracer : tracers) tracer.close();
    }
}
