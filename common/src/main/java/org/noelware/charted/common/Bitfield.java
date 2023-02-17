/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright (c) 2022-2023 Noelware, LLC. <team@noelware.org>
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

import java.util.*;
import org.jetbrains.annotations.NotNull;

/**
 * Represents a field for holding bits for RBAC permissions. The server uses a bitfield to control
 * over what RBAC permissions a user has or not. This is also used for organization flags (for experimental reasons),
 * API key scopes, and repository flags (i.e, if it is private or not)
 *
 * @author Noel
 * @since 31.05.2022
 */
public class Bitfield {
    private final Map<String, Long> flags;
    private long bits;

    /**
     * Initializes an empty bitfield map with 0 bits and no flags
     * attached for the bitfield map itself.
     */
    public Bitfield() {
        this(0, new HashMap<>());
    }

    /**
     * Initializes x amount of bits for the bitfield map with no flags
     * attached for metadata.
     *
     * @param bits The bits to use
     */
    public Bitfield(long bits) {
        this(bits, new HashMap<>());
    }

    /**
     * Initializes the given bits for the bitfield map with flags for metadata
     * purposes and easy readability.
     *
     * @param bits The bit to use
     * @param flags Metadata flags.
     */
    public Bitfield(long bits, Map<String, Long> flags) {
        this.flags = Collections.unmodifiableMap(flags);
        this.bits = bits;
    }

    /**
     * Returns the initialized bits that this {@link Bitfield} is holding.
     */
    public long bits() {
        return this.bits;
    }

    /**
     * Returns the metadata flags that this {@link Bitfield} is using.
     */
    @NotNull
    public Map<String, Long> flags() {
        return this.flags;
    }

    /**
     * Returns a {@link List<Long>} of all the flags' bits available.
     */
    public List<Long> asList() {
        return flags.values().stream().toList();
    }

    /**
     * Returns all the available flags that this {@link Bitfield} has access towards
     */
    public List<String> enabledFlags() {
        return flags.keySet().stream().filter(this::has).toList();
    }

    /**
     * Adds all the bits from {@link #asList()} to the bitfield map.
     * @return This instance to chain methods.
     */
    public @NotNull Bitfield addAll() {
        long total = 0;
        for (long bit : asList()) {
            total |= bit;
        }

        this.bits |= total;
        return this;
    }

    /**
     * Adds a bit to the bitfield map by the metadata flag that was
     * registered in this {@link Bitfield}.
     *
     * @param flag The flag name to use
     */
    public @NotNull Bitfield add(@NotNull String flag) {
        final var bit = flags.get(flag);
        return bit == null ? this : add(bit);
    }

    /**
     * Adds multiple flags from the metadata flag map to the registered
     * bitmap.
     *
     * @param flags The flag names to use
     */
    public @NotNull Bitfield add(String... flags) {
        for (String flag : flags) {
            add(flag);
        }

        return this;
    }

    /**
     * Adds a list of bits to the bitmap.
     * @param bits The bits to register.
     */
    public @NotNull Bitfield add(long... bits) {
        long total = 0;
        for (long bit : bits) {
            total |= bit;
        }

        this.bits |= total;
        return this;
    }

    /**
     * Checks if the bit is in the bitmap.
     * @param bit The bit to check
     */
    public boolean has(long bit) {
        return (this.bits & bit) != 0;
    }

    /**
     * Checks if the metadata flag specified in this bitfield is in the bitmap.
     * @param flag The flag name to check.
     */
    public boolean has(String flag) {
        final var bit = flags.get(flag);
        return bit != null && has(bit);
    }

    /**
     * Removes a bit to the bitfield map by the metadata flag that was
     * registered in this {@link Bitfield}.
     *
     * @param flag The flag name to use
     */
    public @NotNull Bitfield remove(@NotNull String flag) {
        final var bit = flags.get(flag);
        return bit == null ? this : remove(bit);
    }

    /**
     * Adds multiple flags from the metadata flag map to the registered
     * bitmap.
     *
     * @param flags The flag names to use
     */
    public @NotNull Bitfield remove(String... flags) {
        for (String flag : flags) {
            remove(flag);
        }

        return this;
    }

    /**
     * Removes a select list of bits from the bitmap.
     * @param bits The bits to remove.
     */
    public @NotNull Bitfield remove(long... bits) {
        long total = 0;
        for (long bit : bits) {
            total |= bit;
        }

        this.bits &= ~total;
        return this;
    }

    /**
     * Checks if the given <code>flag</code> is available or not
     * @param flag The flag to check
     * @return bool if the <code>flag</code> given is available
     */
    public boolean available(String flag) {
        return flags.containsKey(flag);
    }

    /**
     * Same as {@link #available(String)}, but with a bit literal
     * @param flag The bit literal to check
     * @return bool if the <code>flag</code> given is available
     */
    public boolean available(long flag) {
        final Optional<Long> available =
                flags.values().stream().filter(f -> f == flag).findFirst();

        return available.isPresent();
    }
}
