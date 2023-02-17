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

package org.noelware.charted.common.tests;

import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.Test;
import org.noelware.charted.common.TimeSpanValue;

public class TimeSpanValueTests {
    @Test
    public void test_ifParsesCorrectly() {
        assertEquals(31557600000L, TimeSpanValue.fromString("1 year"));
        assertEquals(604800000L, TimeSpanValue.fromString("1 week"));
        assertEquals(172800000L, TimeSpanValue.fromString("2 days"));
        assertEquals(18000000L, TimeSpanValue.fromString("5hr"));
        assertEquals(22000L, TimeSpanValue.fromString("22 seconds"));
        assertEquals(25L, TimeSpanValue.fromString("25 milliseconds"));
    }

    @Test
    public void test_checkIfParsesCorrectlyByString() {
        assertEquals("1 year", TimeSpanValue.fromLong(31557600000L, true));
        assertEquals("1w", TimeSpanValue.fromLong(604800000L, false));
        assertEquals("2 days", TimeSpanValue.fromLong(172800000L, true));
        assertEquals("5 hours", TimeSpanValue.fromLong(18000000L, true));
        assertEquals("12min", TimeSpanValue.fromLong(720000L, false));
        assertEquals("22 seconds", TimeSpanValue.fromLong(22000L, true));
        assertEquals("22ms", TimeSpanValue.fromLong(22L, false));
    }
}
