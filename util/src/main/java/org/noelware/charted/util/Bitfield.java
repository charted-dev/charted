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

package org.noelware.charted.util;

import java.util.*;

public class Bitfield {
  private Map<String, Long> flags;

  private long bits;

  public Bitfield(Map<String, Long> flags) {
    this.flags = flags;
    this.bits = 0L;
  }

  public Bitfield(Map<String, Long> flags, long... bits) {
    this.flags = flags;
    this.bits = Arrays.stream(bits).reduce(0, (_i, u) -> u);
  }

  public Bitfield(long bits) {
    this.bits = bits;
  }

  public long getBits() {
    return this.bits;
  }

  public Map<String, Long> getFlags() {
    return this.flags;
  }

  public Bitfield remove(long... bits) {
    var total = 0L;
    for (long bit : bits) {
      total |= bit;
    }

    this.bits &= ~total;
    return this;
  }

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

  public long missing(long... bits) {
    return new Bitfield(this.flags, bits)
        .toList().stream()
            .reduce(
                0L,
                (acc, curr) -> {
                  if (has(acc)) {
                    return acc + curr;
                  }

                  return acc;
                });
  }

  public boolean has(long bit) {
    return (this.bits & bit) == bit;
  }
}
