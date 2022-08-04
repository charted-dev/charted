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

package org.noelware.charted.common;

import java.util.concurrent.atomic.AtomicBoolean;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

/**
 * Represents a value that can be set once, retrieved at any time.
 *
 * @param <T> The resolvent type.
 * @author Noel
 * @since 2022.06.14
 */
public class SetOnceGetValue<T> {
    private final AtomicBoolean hasSet = new AtomicBoolean(false);
    private @Nullable T value;

    public @NotNull T getValue() {
        if (value == null) throw new IllegalStateException("Cannot retrieve the value due to it not being set.");
        return value;
    }

    public @Nullable T getValueOrNull() {
        return value;
    }

    public void setValue(@NotNull T value) {
        if (hasSet.compareAndSet(false, true)) {
            this.value = value;
        }
    }

    public boolean wasSet() {
        return this.hasSet.get();
    }

    @Override
    public int hashCode() {
        return value == null ? 0 : value.hashCode();
    }

    @Override
    public boolean equals(Object value) {
        if (value == null) return false;
        if (!(value instanceof SetOnceGetValue<?> setter)) return false;

        if (setter.value == null) return false;
        return value.equals(setter.value);
    }

    @Override
    public String toString() {
        return String.format("charted.SetOnceGetValue(%s)", value == null ? "<uninit>" : value.toString());
    }
}
