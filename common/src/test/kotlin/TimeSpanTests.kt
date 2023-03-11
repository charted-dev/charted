/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

package org.noelware.charted.common.tests

import org.junit.jupiter.api.Test
import org.noelware.charted.common.TimeSpan
import kotlin.test.assertEquals

class TimeSpanTests {
    @Test
    fun `if TimeSpan#ofString can parse correctly`() {
        assertEquals(31557600000L, TimeSpan.ofString("1 year").value)
        assertEquals(604800000L, TimeSpan.ofString("1 week").value)
        assertEquals(18000000L, TimeSpan.ofString("5hrs").value)
        assertEquals(22000L, TimeSpan.ofString("22 seconds").value)
        assertEquals(25L, TimeSpan.ofString("25 milliseconds").value)
    }

    @Test
    fun `if TimeSpan dot toString(bool) can be parsed correctly`() {
        assertEquals("1 year", TimeSpan(31557600000L).toString(true))
        assertEquals("1w", TimeSpan(604800000L).toString())
        assertEquals("2 days", TimeSpan(172800000L).toString(true))
        assertEquals("5 hours", TimeSpan(18000000L).toString(true))
        assertEquals("12m", TimeSpan(720000L).toString())
        assertEquals("22s", TimeSpan(22000L).toString())
        assertEquals("22ms", TimeSpan(22L).toString())
    }
}
