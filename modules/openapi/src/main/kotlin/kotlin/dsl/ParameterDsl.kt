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
import io.swagger.v3.oas.models.media.Schema
import io.swagger.v3.oas.models.parameters.HeaderParameter
import io.swagger.v3.oas.models.parameters.Parameter
import io.swagger.v3.oas.models.parameters.PathParameter
import io.swagger.v3.oas.models.parameters.QueryParameter
import org.noelware.charted.annotations.ChartedDsl
import org.noelware.charted.common.Buildable
import org.noelware.charted.common.extensions.setonce.getValue
import org.noelware.charted.common.extensions.setonce.setValue
import org.noelware.charted.modules.openapi.computeSchema
import kotlin.reflect.KType

/**
 * The kind of parameter this is
 */
enum class ParameterKind {
    Query,
    Header,
    Path
}

@ChartedDsl
interface ParameterDsl: SchemaSupport {
    /**
     * If this parameter is deprecated or not.
     */
    var deprecated: Boolean

    /**
     * If this parameter is required or not.
     */
    var required: Boolean

    /**
     * Description about this parameter
     */
    var description: String?

    /**
     * The style of this parameter
     */
    var style: Parameter.StyleEnum?

    /**
     * Kind of parameter this is
     */
    var kind: ParameterKind?

    /**
     * The name of the parameter
     */
    var name: String
}

class ParameterDslBuilder: ParameterDsl, Buildable<Parameter> {
    private val _deprecated: SetOnce<Boolean> = SetOnce()
    private val _description: SetOnce<String> = SetOnce()
    private val _required: SetOnce<Boolean> = SetOnce()
    private val _schema: SetOnce<Schema<*>> = SetOnce()
    private val _style: SetOnce<Parameter.StyleEnum> = SetOnce()
    private val _kind: SetOnce<ParameterKind> = SetOnce()
    private val _name: SetOnce<String> = SetOnce()

    override var deprecated: Boolean
        get() = _deprecated.valueOrNull ?: false
        set(value) {
            _deprecated.value = value
        }

    override var required: Boolean
        get() = _required.valueOrNull ?: true
        set(value) {
            _required.value = value
        }

    override var description: String? by _description
    override var style: Parameter.StyleEnum? by _style
    override var kind: ParameterKind? by _kind
    override var name: String
        get() = _name.value
        set(value) {
            _name.value = value
        }

    override fun schema(type: KType) {
        _schema.value = computeSchema(type)
    }

    override fun build(): Parameter {
        val param = when (_kind.valueOrNull) {
            ParameterKind.Header -> HeaderParameter()
            ParameterKind.Query -> QueryParameter()
            ParameterKind.Path -> PathParameter()
            else -> Parameter()
        }

        return param.apply {
            _description.valueOrNull?.let { description(it) }
            _schema.valueOrNull?.let { schema(it) }
            _style.valueOrNull?.let { style(it) }

            deprecated(this@ParameterDslBuilder.deprecated)
            required(this@ParameterDslBuilder.required)
            name(this@ParameterDslBuilder.name)
        }
    }
}
