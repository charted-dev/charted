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

package org.noelware.charted.configuration.kotlin.dsl.features

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * List of stable features that can be enabled.
 */
@Serializable
public enum class Feature {
    /**
     * Enables the Docker Registry feature. The Docker registry feature is a feature to
     * use an OCI-based registry (like Docker registry) for your Helm chart destination.
     *
     * This will use charted-server's home-made implementation to keep data structures
     * easily aligned with the server's architecture. You can enable the [ExperimentalFeature.ExternalOciRegistry]
     * feature to use an external registry instead.
     */
    @SerialName("docker_registry")
    DockerRegistry,

    /**
     * Enables the Audit Logs feature. Audit logs are a way to introspect what you
     * or a team member is doing.
     */
    @SerialName("audit_logs")
    AuditLogging,

    /**
     * Enables the Webhooks feature. Webhooks are a way to introspect any event that occurs
     * and posts it in a Discord channel, Slack channel, HTTP webhook, or more.
     *
     * This will also register the `webhook_settings` PostgreSQL table to configure the webhook
     * settings per repository, organization, or user.
     */
    @SerialName("webhooks")
    Webhooks,

    /**
     * Wildcard feature, that will enable all features except the [DockerRegistry] one
     * since that will require the user to enable it if they want to and not force it
     * if a wildcard is present.
     */
    @SerialName("*")
    Wildcard
}

/**
 * Determines if a specific stable feature is enabled or not.
 */
public infix fun List<Feature>.enabled(feature: Feature): Boolean = isWildcard() || any { it == feature }

/**
 * Determine if the given list of [Feature] is a wildcard, so we list
 * all the options in the key-set instead of specific keys.
 */
public fun List<Feature>.isWildcard(): Boolean {
    if (isEmpty()) return false
    if (size == 1) return first() == Feature.Wildcard

    return any { it == Feature.Wildcard }
}
