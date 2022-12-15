/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

package org.noelware.charted.modules.webhooks.events

import kotlinx.datetime.LocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.modules.webhooks.types.WebhookOriginKind

/**
 * Represents the structure of the webhook event packet that is sent to ClickHouse.
 * @param originKind The [origin][origin] [kind][WebhookOriginKind] where this packet belongs to
 * @param firedAt    [LocalDateTime] when the webhook event was fired at
 * @param origin     origin snowflake ID
 * @param data       The data that is stored that contains extra information
 * @param id         The snowflake of this webhook event
 */
@Serializable
class WebhookEvent<T>(
    @SerialName("origin_kind")
    val originKind: WebhookOriginKind,

    @SerialName("fired_at")
    val firedAt: LocalDateTime,
    val origin: Long,
    val data: T,
    val id: Long
)
