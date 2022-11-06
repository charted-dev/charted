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

package org.noelware.charted.modules.avatars.tests

import org.junit.jupiter.api.Test
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.modules.avatars.DefaultAvatarModule
import kotlin.test.assertEquals

class AvatarModuleTests {
    private val module = DefaultAvatarModule(
        Config.Builder().apply {
            jwtSecretKey = "woah"
            baseUrl = "https://charts.noelware.org/api"
        }.build()
    )

    @Test
    fun gravatar() {
        assertEquals("https://secure.gravatar.com/avatar/b4b650811eacc8caee5c84200d800cef.png", module.gravatar("cutie@floofy.dev"))
    }

    @Test
    fun identicons() {
        assertEquals("https://avatars.dicebear.com/api/identicon/69420.svg", module.identicons(69420))
    }

    @Test
    fun charted() {
        assertEquals("https://charts.noelware.org/api/users/69420/avatars/current.png", module.charted(69420))
    }
}
