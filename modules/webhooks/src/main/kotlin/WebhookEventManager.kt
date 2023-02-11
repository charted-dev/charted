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

package org.noelware.charted.modules.webhooks

import kotlinx.serialization.json.JsonObject
import org.noelware.charted.modules.webhooks.events.WebhookEvent
import org.noelware.charted.modules.webhooks.types.WebhookOriginKind
import kotlin.reflect.KClass

/**
 * Represents the manager for handling webhook events into ClickHouse.
 */
interface WebhookEventManager {
    /**
     * Gets all the [webhook events][WebhookEvent] from the given [webhook origin][WebhookOriginKind].
     * @param origin ([WebhookOriginKind] -> repo/org id)
     * @return list of [WebhookEvent] objects
     */
    suspend fun getAll(origin: Pair<WebhookOriginKind, Long>): List<WebhookEvent<JsonObject>>

    /**
     * Returns a specific webhook event from the [inner] class to serialize from. Returns `null`
     * if the packet wasn't found.
     *
     * @param inner [KClass] to serialize the packet from
     * @param origin ([WebhookOriginKind] -> repo/org id)
     */
    suspend fun <T : Any> get(inner: KClass<T>, origin: Pair<WebhookOriginKind, Long>): WebhookEvent<T>?

    /**
     * Creates a new [WebhookEvent] packet.
     * @param origin [WebhookOriginKind] -> repo/org id
     * @param data Any data to attach to this packet
     */
    suspend fun <T> create(
        origin: Pair<WebhookOriginKind, Long>,
        data: T?
    ): WebhookEvent<T>
}

/**
 * Returns a specific webhook event from the [inner] class to serialize from. Returns `null`
 * if the packet wasn't found.
 *
 * @param origin ([WebhookOriginKind] -> repo/org id)
 */
suspend inline fun <reified T : Any> WebhookEventManager.get(origin: Pair<WebhookOriginKind, Long>): WebhookEvent<T>? = get(T::class, origin)
