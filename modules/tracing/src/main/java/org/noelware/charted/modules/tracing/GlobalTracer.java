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

import dev.floofy.utils.java.SetOnce;
import org.jetbrains.annotations.NotNull;

public class GlobalTracer {
    private static final SetOnce<Tracer> globalInstance = new SetOnce<>();

    /**
     * @return global {@link Tracer} instance, cannot be null
     * @throws IllegalStateException If the {@link GlobalTracer} wasn't previously set
     */
    @NotNull
    public static Tracer getInstance() {
        return globalInstance.getValue();
    }

    /**
     * Sets a global tracer, if a tracer was already set, then it will
     * not do anything.
     *
     * @param tracer The tracer to set
     */
    public static void set(Tracer tracer) {
        globalInstance.setValue(tracer);
    }
}
