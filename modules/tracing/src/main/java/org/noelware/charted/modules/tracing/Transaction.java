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

import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

public interface Transaction extends AutoCloseable {
    /**
     * Creates a span in this transaction.
     * @param name Name for this {@link Span} to be referred as
     * @param operation The type of operation this span was created for
     * @return created {@link Span}.
     */
    @NotNull
    Span createSpan(@NotNull String name, @Nullable String operation);

    /**
     * Creates a {@link Span} in this transaction.
     * @param name The name that this span is referred as
     * @return created {@link Span}.
     */
    @NotNull
    Span createSpan(@NotNull String name);

    /**
     * @return the parent tracer this transaction belongs to
     */
    @NotNull
    Tracer tracer();

    @Nullable
    String operation();

    @NotNull
    String name();
}

/*
public interface Transaction extends Closeable {
@NotNull
Span createSpan(@NotNull String name, @Nullable String operation);
    @NotNull
    Tracer getTracer();
    void complete(@NotNull Throwable exception);
    void complete();
    @Nullable
    String getOperation();
    @NotNull
    String getName();
}

 */
