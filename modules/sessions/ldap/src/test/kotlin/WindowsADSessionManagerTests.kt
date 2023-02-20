/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.testing.sessions.ldap

import org.junit.jupiter.api.condition.EnabledIf

/**
 * To run these tests, you will need to have Windows Server running with the proper
 * environment variables:
 *
 *  - `TESTS_WINDOWS_AD_ENABLE=true` to let JUnit know to run these tests
 *  - `TESTS_WINDOWS_AD_DOMAIN=...` - the domain to use when testing
 */
@EnabledIf("org.noelware.charted.testing.sessions.ldap.WindowsADSessionManagerTests.isEnabled")
open class WindowsADSessionManagerTests {
    companion object {
        /**
         * @return if this test bed should be enabled
         */
        @Suppress("unused")
        @JvmStatic
        fun isEnabled(): Boolean = System.getenv("TESTS_WINDOWS_AD_ENABLE") == "true" && System.getenv("TESTS_WINDOWS_AD_DOMAIN") != null
    }
}
