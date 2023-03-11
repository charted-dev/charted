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
import io.ktor.http.*
import io.swagger.v3.oas.models.Operation
import io.swagger.v3.oas.models.parameters.Parameter
import io.swagger.v3.oas.models.parameters.RequestBody
import io.swagger.v3.oas.models.responses.ApiResponse
import io.swagger.v3.oas.models.responses.ApiResponses
import org.noelware.charted.annotations.ChartedDsl
import org.noelware.charted.common.Buildable
import org.noelware.charted.common.extensions.setonce.getValue
import org.noelware.charted.common.extensions.setonce.setValue

@ChartedDsl
interface OperationDsl {
    /**
     * Tiny description about this [operation][OperationDsl].
     */
    var description: String?

    /**
     * Whether if this [OperationDsl] is deprecated or not
     */
    var deprecated: Boolean

    /**
     * Sets the request body, if needed.
     */
    fun requestBody(block: RequestBodyDsl.() -> Unit = {})

    /**
     * Adds a [ApiResponse] to this [OperationDsl].
     */
    fun response(status: HttpStatusCode, block: ResponseDsl.() -> Unit = {})

    /**
     * Adds a query parameter to this [OperationDsl]
     * @param block [ParameterDsl] object
     */
    fun queryParameter(block: ParameterDsl.() -> Unit = {})

    /**
     * Adds a path parameter to this [OperationDsl]
     * @param block [ParameterDsl] object
     */
    fun pathParameter(block: ParameterDsl.() -> Unit = {})

    /**
     * Adds a header parameter to this [OperationDsl]
     * @param block [ParameterDsl] object
     */
    fun header(block: ParameterDsl.() -> Unit = {})

    /**
     * Adds multiple tags to this [operation][OperationDsl].
     * @param tags List of tags to append
     */
    fun tags(vararg tags: String)
}

class OperationDslBuilder: OperationDsl, Buildable<Operation> {
    private val _description: SetOnce<String> = SetOnce()
    private val _requestBody: SetOnce<RequestBody> = SetOnce()
    private val _parameters: MutableList<Parameter> = mutableListOf()
    private val _deprecated: SetOnce<Boolean> = SetOnce()
    private val _responses: ApiResponses = ApiResponses()
    private val _tags: MutableList<String> = mutableListOf()

    override var description: String? by _description
    override var deprecated: Boolean
        get() = _deprecated.valueOrNull ?: false
        set(value) {
            _deprecated.value = value
        }

    override fun requestBody(block: RequestBodyDsl.() -> Unit) {
        _requestBody.value = RequestBodyDslBuilder().apply(block).build()
    }

    override fun response(status: HttpStatusCode, block: ResponseDsl.() -> Unit) {
        _responses.addApiResponse(status.value.toString(), ResponseDslBuilder().apply(block).build())
    }

    override fun queryParameter(block: ParameterDsl.() -> Unit) {
        _parameters.add(
            ParameterDslBuilder().apply {
                kind = ParameterKind.Query
                return@apply block()
            }.build(),
        )
    }

    override fun pathParameter(block: ParameterDsl.() -> Unit) {
        _parameters.add(
            ParameterDslBuilder().apply {
                kind = ParameterKind.Path
                return@apply block()
            }.build(),
        )
    }

    override fun header(block: ParameterDsl.() -> Unit) {
        _parameters.add(
            ParameterDslBuilder().apply {
                kind = ParameterKind.Header
                return@apply block()
            }.build(),
        )
    }

    override fun tags(vararg tags: String) {
        for (tag in tags) _tags.add(tag)
    }

    override fun build(): Operation = Operation().apply {
        _description.valueOrNull?.let { description(it) }
        deprecated(this@OperationDslBuilder.deprecated)

        for (tag in _tags.distinct()) addTagsItem(tag)
        for (param in _parameters) addParametersItem(param)

        responses(this@OperationDslBuilder._responses)
    }
}
