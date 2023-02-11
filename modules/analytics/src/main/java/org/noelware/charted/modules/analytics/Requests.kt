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

package org.noelware.charted.modules.analytics

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

class Requests {
    @Serializable
    class InitRequest(val addr: String)

    @Serializable
    class InitResponse(val uuid: String, @SerialName("pub_key") val pubKey: String)

    @Serializable
    class FinalizeRequest(@SerialName("api_token") val apiToken: String)
}
