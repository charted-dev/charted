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

package org.noelware.charted.invitations

import com.github.mustachejava.DefaultMustacheFactory
import com.github.mustachejava.MustacheResolver
import dev.floofy.utils.kotlin.ifNotNull
import dev.floofy.utils.slf4j.logging
import io.lettuce.core.SetArgs
import kotlinx.coroutines.*
import kotlinx.coroutines.future.await
import kotlinx.serialization.json.Json
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.IRedisClient
import org.noelware.charted.common.RandomGenerator
import org.noelware.charted.common.data.Config
import org.noelware.charted.database.entities.UserEntity
import org.noelware.charted.database.models.Organization
import org.noelware.charted.database.models.Repository
import org.noelware.charted.email.EmailService
import java.io.File
import java.io.StringWriter
import kotlin.time.Duration.Companion.minutes
import kotlin.time.Duration.Companion.seconds

class DefaultInvitationManager(
    private val email: EmailService? = null,
    private val config: Config,
    private val redis: IRedisClient,
    private val json: Json
): InvitationManager {
    private val mustache = DefaultMustacheFactory(TemplatesResolver)
    private val invitationJobs: MutableMap<String, Job> = mutableMapOf()
    private val log by logging<DefaultInvitationManager>()

    init {
        val expirations = runBlocking { redis.commands.keys("charted:invitations:*").await() }
        for (key in expirations) {
            val ttl = runBlocking { redis.commands.ttl(key).await() }
            if (ttl == -2L) continue
            if (ttl == -1L) {
                runBlocking {
                    redis.commands.del(key).await()
                }
            } else {
                invitationJobs[key.split(":")[2]] = ChartedScope.launch {
                    delay(ttl.seconds.inWholeMilliseconds)
                    redis.commands.del(key)
                }
            }
        }
    }

    /**
     * Sends out an invitation to join a repository if the email service is registered and returns the URL
     * to accept the invitation.
     */
    override suspend fun sendRepositoryInvitation(repository: Repository, user: UserEntity): String {
        log.info("Sending out invitation to user [${user.id}]")

        val code = RandomGenerator.generate(6)
        val invitation = Invitation(InvitationAction.JOIN_REPOSITORY, user.id.value, code, repository.id)

        redis.commands.set(
            "charted:invitations:$code",
            json.encodeToString(Invitation.serializer(), invitation),
            SetArgs().ex(15.minutes.inWholeSeconds)
        ).await()

        invitationJobs[code] = ChartedScope.launch {
            delay(15.minutes.inWholeMilliseconds)
            redis.commands.del("charted:invitations:$code").await()
        }

        val url = if (config.baseUrl != null) "${config.baseUrl}/repositories/members/invitations/$code"
        else "http://${config.server.host}:${config.server.port}/repositories/members/invitations/$code"

        return if (email == null) url else {
            sendEmail(url, user.email, repository.name, InvitationAction.JOIN_REPOSITORY)
            return url
        }
    }

    /**
     * Sends out an invitation to join a repository if the email service is registered and returns the URL
     * to accept the invitation.
     */
    override suspend fun sendOrganizationInvitation(organization: Organization, user: UserEntity): String {
        log.info("Sending out invitation to user [${user.id}]")

        val code = RandomGenerator.generate(6)
        val invitation = Invitation(InvitationAction.JOIN_ORGANIZATION, user.id.value, code, organization.id)

        redis.commands.set(
            "charted:invitations:$code",
            json.encodeToString(Invitation.serializer(), invitation),
            SetArgs().ex(15.minutes.inWholeSeconds)
        ).await()

        invitationJobs[code] = ChartedScope.launch {
            delay(15.minutes.inWholeMilliseconds)
            redis.commands.del("charted:invitations:$code").await()
        }

        val url = if (config.baseUrl != null) "${config.baseUrl}/organizations/members/invitations/$code"
        else "http://${config.server.host}:${config.server.port}/organizations/members/invitations/$code"

        return if (email == null) url else {
            sendEmail(url, user.email, organization.displayName ?: organization.name, InvitationAction.JOIN_REPOSITORY)
            return url
        }
    }

    /**
     * Validates the invitation to check if it's still available as a boolean value.
     * @param id The repository or organization ID.
     * @param userID The user's ID that the invitation belongs to.
     * @param code The invitation's unique identifier that the invitation was attached to.
     * @return If the invitation is still valid, `true` otherwise `false`.
     */
    override suspend fun validateInvitation(id: Long, userID: Long, code: String): Boolean {
        return redis
            .commands
            .get("charted:invitations:$code")
            .await()
            .ifNotNull { true } ?: return false
    }

    /**
     * Closes this stream and releases any system resources associated
     * with it. If the stream is already closed then invoking this
     * method has no effect.
     *
     * As noted in [AutoCloseable.close], cases where the
     * close may fail require careful attention. It is strongly advised
     * to relinquish the underlying resources and to internally
     * *mark* the `Closeable` as closed, prior to throwing
     * the `IOException`.
     *
     * @throws java.io.IOException if an I/O error occurs
     */
    override fun close() {
        for (job in invitationJobs.values) job.cancel()
    }

    private suspend fun sendEmail(
        url: String,
        recipient: String,
        name: String,
        action: InvitationAction
    ) {
        val subject = "You were invited to join the $name ${if (action == InvitationAction.JOIN_REPOSITORY) "repository" else "organization"}!"
        val writer = StringWriter()
        val m = mustache.compile("member-invitation.html")
        val ctx = mapOf(
            "url" to url,
            "type" to if (action == InvitationAction.JOIN_REPOSITORY) "repository" else "organization"
        )

        withContext(Dispatchers.IO) {
            m.execute(writer, ctx).flush()
        }

        val html = writer.toString()
        email!!.sendEmail(recipient, subject, html)
    }

    companion object {
        val TemplatesResolver = MustacheResolver { path ->
            File("./assets/templates/$path").reader(Charsets.UTF_8)
        }
    }
}
