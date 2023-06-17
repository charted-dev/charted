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

package org.noelware.charted.models.repositories

import com.fasterxml.jackson.annotation.JsonProperty
import io.swagger.v3.oas.annotations.media.Schema
import kotlinx.datetime.LocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.models.VersionConstraint
import org.noelware.charted.snowflake.Snowflake

/**
 * Represents a Repository Release resource. Releases are a way to group Helm chart
 * versions as a singular tag that can be fetched from the API server.
 *
 * Any repository can have an unlimited amount of tags, but tags cannot clash into
 * each other, so there is no more than release tag per release. Each tag has to
 * be a SemVer 2 comformant string and is validated by the API server.
 */
@Schema(
    description = "Represents a Repository Release resource. Releases are a way to group Helm chart" +
        " versions as a singular tag that can be fetched from the API server." +
        "\n" +
        "Any repository can have an unlimited amount of tags, but tags cannot clash into" +
        " each other, so there is no more than release tag per release. Each tag has to" +
        " be a SemVer 2 comformant string and is validated by the API server.",
)
@Serializable
public data class RepositoryRelease(
    /**
     * Markdown-formatted string that contains a changelog of this release.
     */
    @get:Schema(description = "Markdown-formatted string that contains a changelog of this release.")
    @JsonProperty("update_text")
    @SerialName("update_text")
    val updateText: String? = null,

    /**
     * Date-time of when this repository release resource was created at.
     */
    @get:Schema(description = "Date-time of when this repository release resource was created at.")
    @JsonProperty("created_at")
    @SerialName("created_at")
    val createdAt: LocalDateTime,

    /**
     * Date-time of when this repository release was last updated at.
     */
    @get:Schema(description = "Date-time of when this repository release was last updated at.")
    @JsonProperty("updated_at")
    @SerialName("updated_at")
    val updatedAt: LocalDateTime,

    /**
     * Tag that this release is represented as. Must be a valid SemVer 2 comformant
     * string.
     */
    @get:Schema(description = "Tag that this release is represented as. Must be a valid SemVer 2 comformant string.", implementation = VersionConstraint::class)
    val tag: String,

    /**
     * Unique identifier to locate this repository resource from the API
     */
    @get:Schema(description = "Unique identifier to locate this repository from the API", implementation = Snowflake::class)
    val id: Long
)
