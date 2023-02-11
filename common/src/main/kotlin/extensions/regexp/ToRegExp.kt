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

package org.noelware.charted.extensions.regexp

import org.noelware.charted.common.RegularExpressions
import org.noelware.charted.common.regexp.RegularExpression

/**
 * Transforms a string into a [RegularExpression] object, calls the [RegularExpressions.getPasswordRegex] method.
 */
public fun String.toPasswordRegex(): RegularExpression = RegularExpressions.getPasswordRegex(this)

/**
 * Transforms a string into a [RegularExpression] object, which calls the [RegularExpressions.getNameRegex] method.
 * @param includeNumbers If the regular expression should include numbers or not. [default=true]
 */
public fun String.toNameRegex(
    includeNumbers: Boolean = true,
    length: Int = 32
): RegularExpression = RegularExpressions.getNameRegex(this, includeNumbers, length)
