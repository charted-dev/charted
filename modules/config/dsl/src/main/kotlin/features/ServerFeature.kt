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

package org.noelware.charted.configuration.kotlin.dsl.features

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Represents all the features the server is allowed to bootstrap. The official server
 * uses the [AUDIT_LOGS] and [WEBHOOKS] features.
 */
@Serializable
@Suppress("ktlint:no-semi")
public enum class ServerFeature {
    /**
     * Enables the Audit Logs feature. Audit logs are a way to introspect what you
     * or a team member is doing. This requires a ClickHouse server to be operated on since
     * it processes massive amount of data at any time.
     */
    @SerialName("audit_logs")
    AUDIT_LOGS,

    /**
     * Enables the Docker Registry feature. The Docker registry feature is a feature to
     * use an OCI-based registry (like Docker registry) for your Helm chart destination.
     *
     * This will not configure the Chart-based engine that is optimized for charted-server in
     * any way. You will need to fill in the `config.oci` configuration before running the server.
     */
    @SerialName("docker_registry")
    DOCKER_REGISTRY,

    /**
     * Enables the Webhooks feature. Webhooks are a way to introspect any event that occurs
     * and posts it in a Discord channel, Slack channel, HTTP webhook, or more. This requires a
     * ClickHouse server to be operated since it'll process massive amounts of data at any time.
     *
     * This will also register the `webhook_settings` PostgreSQL table to configure the webhook
     * settings per repository, organization, or user.
     */
    @SerialName("webhooks")
    WEBHOOKS;
}
