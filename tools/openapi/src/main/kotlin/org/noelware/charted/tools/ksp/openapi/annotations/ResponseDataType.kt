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

package org.noelware.charted.tools.ksp.openapi.annotations

/**
 * Represents a data type that is used in a response of a REST handler.
 * @param statusCode The status code of this response type.
 * @param description The description of this response type.
 * @param contentTypes A list of content types that this response type uses.
 */
@Target(AnnotationTarget.CLASS)
annotation class ResponseDataType(
    val statusCode: Int,
    val description: String = "",
    val contentTypes: Array<String> = []
)
