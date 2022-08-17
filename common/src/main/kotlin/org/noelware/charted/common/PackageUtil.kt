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

package org.noelware.charted.common

import dev.floofy.utils.kotlin.humanize
import io.github.z4kn4fein.semver.toVersion
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import kotlinx.datetime.Instant
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.contentOrNull
import kotlinx.serialization.json.jsonPrimitive

/**
 * Environment variable to disable the [updateNotification] handler from doing anything.
 */
private const val NOTIFICATION_ENVIRONMENT_VARIABLE = "CHARTED_DISABLE_UPDATE_NOTIFICATION"

/**
 * Returns the URL for fetching the latest release.
 */
private const val GITHUB_RELEASES_URL = "https://api.github.com/repos/charted-dev/charted/releases/latest"

/**
 * Shows a mini "updater notification" if the server is running on an older version. You can
 * disable this with the `CHARTED_DISABLE_UPDATE_NOTIFICATION` to a boolean-ish value.
 */
suspend fun updateNotification(httpClient: HttpClient) {
    val env = System.getenv(NOTIFICATION_ENVIRONMENT_VARIABLE)
    if (env != null && env.matches("^(no|false|0)$".toRegex())) return

    val res = httpClient.get(GITHUB_RELEASES_URL) {
        header("Accept", "application/vnd.github+json")
    }

    val body: JsonObject = res.body()

    // This might happen if this is a fork of charted-server, or it is
    // a non-supported version of charted-server not by Noelware. So,
    // let's just skip it (for now)
    if (res.status.value == 404 && body["message"]?.jsonPrimitive?.contentOrNull != null) {
        return
    }

    val url = body["html_url"]!!.jsonPrimitive.content
    val tag = body["tag_name"]!!.jsonPrimitive.content
    val createdAt = Instant.parse(body["published_at"]!!.jsonPrimitive.content)

    val version = ChartedInfo.version.toVersion(false)
    val newVersion = tag.toVersion(false)

    // If it is equal or less than the current version, do not
    // run the notification listener.
    if (newVersion <= version) return

    // TODO: make this pretty :)
    println("❗ New version of charted-server is available! Available since ${(System.currentTimeMillis() - createdAt.toEpochMilliseconds()).humanize(true)} ago.")
    println("  $version -> $newVersion")
    println("  Changelog: $url")
    println("  Read the changelog on how to upgrade!")
}
