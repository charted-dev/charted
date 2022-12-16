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

import org.junit.jupiter.api.Test;
import org.noelware.charted.common.SetOnce;

public class SetOnceTests {
    @Test
    public void test_valueByDefaultIsNull() {
        final SetOnce<String> setter = new SetOnce<>();

        assertNull(setter.getValueOrNull());
        assertThrows(IllegalStateException.class, setter::getValue);
    }

    @Test
    public void test_setValue() {
        final SetOnce<String> setter = new SetOnce<>();
        assertFalse(setter.wasSet());

        setter.setValue("owo da uwu");
        assertTrue(setter.wasSet());
        assertEquals("owo da uwu", setter.getValue());
        assertNotEquals("uwu da owo", setter.getValue());

        setter.setValue("heck");
        assertTrue(setter.wasSet());
        assertEquals("owo da uwu", setter.getValue());
        assertNotEquals("heck", setter.getValue());
    }
}
