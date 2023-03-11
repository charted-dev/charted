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
import org.noelware.charted.common.ByteSizeValue
import kotlin.test.assertEquals

class ByteSizeValueTests {
    @Test
    fun `does ByteSizeValue#ofString resolve correctly`() {
        assertEquals(16492674416640L, ByteSizeValue.ofString("15tb").value)
        assertEquals(26843545600L, ByteSizeValue.ofString("25 gigabytes").value)
        assertEquals(262144000L, ByteSizeValue.ofString("250MB").value)
        assertEquals(1047552L, ByteSizeValue.ofString("1023kb").value)
        assertEquals(15L, ByteSizeValue.ofString("15b").value)
    }
}
