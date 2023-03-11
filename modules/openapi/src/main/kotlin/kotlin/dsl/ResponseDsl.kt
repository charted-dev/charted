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

package org.noelware.charted.modules.openapi.kotlin.dsl

import dev.floofy.utils.java.SetOnce
import io.swagger.v3.oas.models.media.Content
import io.swagger.v3.oas.models.responses.ApiResponse
import org.noelware.charted.annotations.ChartedDsl
import org.noelware.charted.common.Buildable
import org.noelware.charted.common.extensions.setonce.getValue
import org.noelware.charted.common.extensions.setonce.setValue

@ChartedDsl
interface ResponseDsl: BodyDsl {
    var description: String?
}

class ResponseDslBuilder: ResponseDsl, Buildable<ApiResponse>, BodyBuilder() {
    private val _description: SetOnce<String> = SetOnce()
    override var description: String? by _description

    override fun build(): ApiResponse = ApiResponse().apply {
        _description.valueOrNull?.let { description(it) }
        content(
            Content().apply {
                for ((contentType, mediaType) in _contentTypes) {
                    addMediaType(contentType, mediaType)
                }
            },
        )
    }
}
