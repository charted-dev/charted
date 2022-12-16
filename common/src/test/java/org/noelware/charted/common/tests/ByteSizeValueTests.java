/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;
import org.noelware.charted.common.ByteSizeValue;

public class ByteSizeValueTests {
    @DisplayName("test if ByteSizeValue#fromString works")
    @Test
    public void test1() {
        assertEquals(16492674416640L, ByteSizeValue.fromString("15tb"));
        assertEquals(26843545600L, ByteSizeValue.fromString("25gb"));
        assertEquals(262144000L, ByteSizeValue.fromString("250mb"));
        assertEquals(1047552L, ByteSizeValue.fromString("1023kb"));
        assertEquals(15L, ByteSizeValue.fromString("15b"));
    }

    @DisplayName("test if ByteSizeValue#fromLong works")
    @Test
    public void test2() {
        assertEquals("15tb", ByteSizeValue.fromLong(16492674416640L));
        assertEquals("25gb", ByteSizeValue.fromLong(26843545600L));
        assertEquals("250mb", ByteSizeValue.fromLong(262144000L));
        assertEquals("1023kb", ByteSizeValue.fromLong(1047552L));
        assertEquals("15b", ByteSizeValue.fromLong(15L));
    }

    @DisplayName("test if ByteSizeValue#fromLong(Long, boolean) works")
    @Test
    public void test3() {
        assertEquals("15tb", ByteSizeValue.fromLong(16492674416640L));
        assertEquals("25 gigabytes", ByteSizeValue.fromLong(26843545600L, true));
        assertEquals("250mb", ByteSizeValue.fromLong(262144000L));
        assertEquals("1023 kilobytes", ByteSizeValue.fromLong(1047552L, true));
        assertEquals("15b", ByteSizeValue.fromLong(15L));
    }
}
