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

package org.noelware.charted.modules.docker.registry

import co.elastic.apm.api.Traced
import com.auth0.jwt.algorithms.Algorithm
import com.auth0.jwt.exceptions.TokenExpiredException
import dev.floofy.utils.slf4j.logging
import io.sentry.Sentry
import kotlinx.coroutines.Job
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.extensions.ifSentryEnabled
import org.noelware.charted.modules.redis.RedisClient

class RegistryAuthorizationPolicyManager(private val redis: RedisClient, private val config: Config) {
    private val expirationJobs: MutableMap<Long, Job> = mutableMapOf()
    private val algorithm: Algorithm = Algorithm.HMAC512(config.jwtSecretKey)
    private val log by logging<RegistryAuthorizationPolicyManager>()

    @Traced
    fun isTokenExpired(token: String): Boolean = try {
        false
    } catch (_: TokenExpiredException) {
        true
    } catch (e: Exception) {
        ifSentryEnabled { Sentry.captureException(e) }
        throw e
    }

    companion object {
        private val REPOSITORY_NAME_REGEX: Regex = "[a-z0-9]+([._-][a-z0-9]+)*(/[a-z0-9]+([._-][a-z0-9]+)*)*".toRegex()
    }
}
