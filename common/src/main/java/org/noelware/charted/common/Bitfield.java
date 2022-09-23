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

    /** Creates a new {@link Bitfield} object with default values. */
    public Bitfield() {
        this(0, new HashMap<>());
    }

    /**
     * Creates a new bitfield with a specific amount and the default "flags"
     *
     * @param bits The bits to represent this {@link Bitfield} object.
     */
    public Bitfield(long bits) {
        this(bits, new HashMap<>());
    }

    /**
     * Creates a new Bitfield object with the flags available and defaults to <code>0</code> as the
     * bit value.
     *
     * @param flags The flags.
     */
    public Bitfield(Map<String, Long> flags) {
        this(0, flags);
    }

    /**
     * Creates a new {@link Bitfield} object with a specified amount of bits and the flags to
     * determine the bits to add via the {@link #add(String)} method.
     *
     * @param bits The amount of bits that this {@link Bitfield} stores.
     * @param flags The flags.
     */
    public Bitfield(long bits, Map<String, Long> flags) {
        this.flags = Collections.unmodifiableMap(flags);
        this.bits = bits;
    }

    /** Returns the remaining bits from this object. */
    public long getBits() {
        return bits;
    }

    /** Returns all the flags used in this bitfield for the {@link #add(String)} method. */
    @NotNull
    public Map<String, Long> getFlags() {
        return flags;
    }

    /** Returns a list of all the values in the flags map. */
    @NotNull
    public List<Long> toList() {
        return flags.values().stream().toList();
    }

    /**
     * Adds a bit via the flag key. This will return a new cloned {@link Bitfield} instance.
     *
     * @param key The flag key to use to determine the bit, returns this {@link Bitfield} instance
     *     if the flag wasn't found.
     * @return The cloned {@link Bitfield} object.
     */
    @NotNull
    public Bitfield add(@NotNull String key) {
        var bit = flags.get(key);
        if (bit == null) return this;

        return add(bit);
    }

    /** Adds all the bits from the flags map, and returns a new cloned {@link Bitfield} object. */
    @NotNull
    public Bitfield addAll() {
        var total = 0L;
        for (long bit : toList()) {
            total |= Math.abs(bit);
        }

        this.bits |= total;
        return this;
    }

    /**
     * Adds an array of bits to a new, cloned {@link Bitfield} object.
     *
     * @param bits The bits to use.
     * @return A cloned {@link Bitfield} object.
     */
    @NotNull
    public Bitfield add(long... bits) {
        var total = 0L;
        for (long bit : bits) {
            total |= Math.abs(bit); // make sure it's positive
        }

        this.bits |= total;
        return this;
    }

    /**
     * Checks if the specified bit is in the bits.
     *
     * @param bit The bit to check.
     */
    public boolean has(long bit) {
        return (bits & bit) != 0;
    }

    /**
     * Checks if the flag specified is in the bitfield
     *
     * @param flag The flag.
     */
    public boolean has(@NotNull String flag) {
        var flagBit = flags.get(flag);
        return flagBit != null && has(flagBit);
    }

    @NotNull
    public Bitfield remove(long... bits) {
        var total = 0L;
        for (long bit : bits) {
            total |= Math.abs(bit); // make it positive
        }

        this.bits &= ~total;
        return this;
    }
}
