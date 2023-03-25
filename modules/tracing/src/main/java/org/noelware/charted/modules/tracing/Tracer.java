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

package org.noelware.charted.modules.tracing;

import java.io.Closeable;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

/**
 * Represents an interface to create transactions based off what tracing backend is being used.
 */
public interface Tracer extends Closeable {
    /**
     * Returns the global {@link Tracer}.
     */
    @NotNull
    static Tracer global() {
        return GlobalTracer.getInstance();
    }

    /**
     * @return global {@link Tracer} if it was set, or <code>null</code> if not.
     */
    @Nullable
    static Tracer globalOrNull() {
        try {
            return global();
        } catch (IllegalStateException ignored) {
            return null;
        }
    }

    /**
     * Sets the global tracer to this {@link Tracer tracer}.
     * @param tracer The tracer to set
     * @throws IllegalStateException If a global tracer was already set.
     */
    static void setGlobal(Tracer tracer) {
        GlobalTracer.set(tracer);
    }

    /**
     * Creates a simple {@link Transaction}.
     * @param name The name of the transaction
     * @param operation Operation, can be null.
     * @return {@link Transaction} that is backed by the configured backend.
     */
    @NotNull
    Transaction createTransaction(@NotNull String name, @Nullable String operation);

    /**
     * Creates a simple {@link Transaction}.
     * @param name The name of the transaction
     * @return {@link Transaction} that is backed by the configured backend.
     */
    @NotNull
    Transaction createTransaction(@NotNull String name);
}
