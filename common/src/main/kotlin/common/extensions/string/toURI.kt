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

@file:JvmName("ToURIExtensionsKt")

package org.noelware.charted.common.extensions.string

import java.net.URI

/**
 * Creates a [URI] object from this string.
 */
public fun String.toUri(): URI = URI.create(this)

/**
 * Same as [toUri] but returns `null` if any errors occur.
 */
public fun String.toUriOrNull(): URI? = try {
    toUri()
} catch (_: Exception) {
    null
}