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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.search.elastic

import org.noelware.charted.search.SearchBuilder

/**
 * Appends the `?pretty` query URL to retrieve a pretty object.
 */
fun SearchBuilder.pretty(value: Boolean = true): SearchBuilder {
    value("pretty", value.toString())
    return this
}

/**
 * Appends the `?error_trace` query URL to retrieve a pretty stacktrace when an error
 * has occurred in the request.
 */
fun SearchBuilder.includeStacktrace(value: Boolean = true): SearchBuilder {
    value("error_trace", value.toString())
    return this
}

/**
 * Adds additional paths to filter out in the request body.
 */
fun SearchBuilder.filterPath(paths: String): SearchBuilder {
    value("filter_path", paths)
    return this
}
