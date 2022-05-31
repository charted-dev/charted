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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.utils;

import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * Represents an array of bits contained into a single class.
 *
 * <p>charted-server makes use of this utility class to provide a simple and intuitive
 * permission-based system that is easy to replicate.
 *
 * <p>This is in Java since Kotlin doesn't support bit operators like <code>&=</code>, <code>&~
 * </code> and such.
 */
public class Bitfield {
  private final Map<String, Long> flags;
  private long bits;

  /** Constructs a simple {@link Bitfield} object. There are no flags or bits attached to this. */
  public Bitfield() {
    this(new HashMap<>(), 0);
  }

  /**
   * Constructs a {@link Bitfield} using a specified amount of bits, but an empty map of flags.
   *
   * @param bits The bits to use
   */
  public Bitfield(long bits) {
    this(new HashMap<>(), bits);
  }

  /**
   * Constructs a {@link Bitfield} object.
   *
   * @param flags The flags to do safe operators for.
   * @param bits The bits to use.
   */
  public Bitfield(Map<String, Long> flags, long bits) {
    this.flags = Collections.unmodifiableMap(flags);
    this.bits = bits;
  }

  /** Returns how many bits are in this current {@link Bitfield} object. */
  public long getBits() {
    return this.bits;
  }

  /** Returns the unmodifiable flags for this {@link Bitfield}. */
  public Map<String, Long> getFlags() {
    return this.flags;
  }

  /**
   * Removes a list of bits from this {@link Bitfield} object.
   *
   * @param bits The bits to remove from.
   * @return this {@link Bitfield} object.
   */
  public Bitfield remove(long... bits) {
    var total = 0L;
    for (long bit : bits) {
      total |= bit;
    }

    this.bits &= ~total;
    return this;
  }

  /**
   * Adds bits to this {@link Bitfield} object.
   *
   * @param bits The bits to use.
   * @return this {@link Bitfield} object.
   */
  public Bitfield add(long... bits) {
    var total = 0L;
    for (long bit : bits) {
      total |= bit;
    }

    this.bits |= total;
    return this;
  }

  public List<Long> toList() {
    return flags.values().stream().toList();
  }

  public boolean has(long bit) {
    return (this.bits & bit) != 0;
  }

  public boolean has(String flag) {
    var f = flags.get(flag);
    if (f != null) {
      return this.has(f);
    }

    return false;
  }
}
