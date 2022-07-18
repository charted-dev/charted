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

package org.noelware.charted.common.tests;

import java.util.HashMap;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;
import org.noelware.charted.common.Bitfield;

public class BitfieldTests {
    @Test
    public void test_ifWeHaveSpecifiedBits() {
        var flags = new HashMap<String, Long>();
        flags.put("PRIVATE", (long) (1));
        flags.put("OWO", (long) (1 << 1));
        flags.put("UWU", (long) (1 << 2));

        var bits = new Bitfield(flags);
        Assertions.assertEquals(bits.getBits(), 0);
        bits.add(1, 1 << 1);

        Assertions.assertEquals(bits.getBits(), 3);
        Assertions.assertTrue(bits.has(1 << 1));
        Assertions.assertFalse(bits.has(1 << 6));
        Assertions.assertTrue(bits.has("OWO"));
        Assertions.assertFalse(bits.has("ice is cute"));
    }

    @Test
    public void test_ifWeCanRemoveBits() {
        var flags = new HashMap<String, Long>();
        flags.put("PRIVATE", (long) (1));
        flags.put("OWO", (long) (1 << 1));
        flags.put("UWU", (long) (1 << 2));

        var bits = new Bitfield(flags);
        Assertions.assertEquals(bits.getBits(), 0);
        bits.add(1, 1 << 1);

        Assertions.assertEquals(bits.remove(1 << 1).getBits(), 1);
        Assertions.assertEquals(bits.remove(1).getBits(), 0);
    }
}
