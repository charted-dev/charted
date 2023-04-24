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

package org.noelware.charted.modules.postgresql.pagination

import com.fasterxml.jackson.annotation.JsonProperty
import io.swagger.v3.oas.annotations.media.Schema
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Represents an object class for paginating entries with-in database controllers. Since all
 * entities have sequential IDs, we will be using cursor-based pagination which has a
 * [PageInfo] block about page information.
 *
 * @param pageInfo The information to check if we can go forwards or backwards.
 * @param items The list of items, which will be 25 entries per-page.
 */
@Schema(description = "Represents a object class for pagination purposes")
@Serializable
data class PaginationObject<T>(
    @get:Schema(description = "Block to indicate if entries can go forward or backwards")
    @JsonProperty("page_info")
    @SerialName("page_info")
    val pageInfo: PageInfo,

    @get:Schema(description = "List of items from the paginated list, which will always be 25 per page.")
    val items: List<T>
) {
    /**
     * Represents information about how to indicate if we can go forward or backwards with
     * a [PaginationObject].
     *
     * @param previous Base64-encoded string of `"previous:<id>"` to go backwards, if we can
     * @param next Base64-encoded string of "next:<id>"` to go forwards, if we can
     */
    @Serializable
    data class PageInfo(
        val previous: String? = null,
        val next: String? = null
    )
}
