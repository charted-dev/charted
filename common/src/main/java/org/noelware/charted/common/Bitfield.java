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

import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import org.jetbrains.annotations.NotNull;

/**
 * Represents an array of bits contained into a single class.
 *
 * <p>charted-server makes use of this utility class to provide a simple and intuitive
 * permission-based system that is easy to replicate.
 *
 * <p>This is in Java since Kotlin doesn't support bit operators like <code>&=</code>, <code>&~
 * </code> and such.
 *
 * @author Noel
 * @since 31.05.2022
 */
public class Bitfield {
    private final Map<String, Long> flags;
    private long bits;

    public Bitfield() {
        this(0, new HashMap<>());
    }

    public Bitfield(long bits) {
        this(bits, new HashMap<>());
    }

    public Bitfield(Map<String, Long> flags) {
        this(0, flags);
    }

    public Bitfield(long bits, Map<String, Long> flags) {
        this.flags = Collections.unmodifiableMap(flags);
        this.bits = bits;
    }

    public long getBits() {
        return bits;
    }

    @NotNull
    public Map<String, Long> getFlags() {
        return flags;
    }

    @NotNull
    public List<Long> toList() {
        return flags.values().stream().toList();
    }

    @NotNull
    public Bitfield add(@NotNull String key) {
        return add(flags.get(key));
    }

    @NotNull
    public Bitfield addAll() {
        for (long i : toList()) {
            add(i);
        }

        return this;
    }

    @NotNull
    public Bitfield add(long... bits) {
        var total = 0L;
        for (long bit : bits) {
            total |= bit;
        }

        this.bits |= total;
        return this;
    }

    public boolean has(long bit) {
        return (bits & bit) != 0;
    }

    public boolean has(@NotNull String flag) {
        var flagBit = flags.get(flag);
        return flagBit != null && has(flagBit);
    }

    @NotNull
    public Bitfield remove(long... bits) {
        var total = 0L;
        for (long bit : bits) {
            total |= bit;
        }

        this.bits &= ~total;
        return this;
    }
}
