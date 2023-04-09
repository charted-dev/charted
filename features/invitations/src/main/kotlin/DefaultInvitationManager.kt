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

package org.noelware.charted.features.invitations

import com.google.protobuf.Struct
import dev.floofy.utils.slf4j.logging
import io.lettuce.core.SetArgs
import kotlinx.coroutines.*
import kotlinx.coroutines.future.await
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import org.noelware.charted.ChartedScope
import org.noelware.charted.launch
import org.noelware.charted.emails.protobufs.v1.SendEmailRequest
import org.noelware.charted.models.repositories.Repository
import org.noelware.charted.models.repositories.RepositoryMember
import org.noelware.charted.models.users.User
import org.noelware.charted.modules.analytics.kotlin.dsl.Struct
import org.noelware.charted.modules.analytics.kotlin.dsl.toGrpcValue
import org.noelware.charted.modules.emails.EmailService
import org.noelware.charted.modules.redis.RedisClient
import java.util.UUID
import kotlin.time.Duration.Companion.minutes
import kotlin.time.DurationUnit
import kotlin.time.toDuration
import kotlin.time.toJavaDuration

class DefaultInvitationManager(
    private val emails: EmailService,
    private val redis: RedisClient,
    private val json: Json
): InvitationManager {
    private val invitationExpirationJobs = mutableMapOf<UUID, Job>()
    private val redisTableKey: String = "charted:member:invitations"
    private val log by logging<DefaultInvitationManager>()

    init {
        log.info("Collecting all invitation information...")
        val invitations = runBlocking { redis.commands.hkeys(redisTableKey).await() } ?: listOf()

        log.info("Collected ${invitations.size} invitation(s) available")
        for (key in invitations) {
            val uuid = UUID.fromString(key)

            log.trace("...found invitation [$key]")
            val ttl = runBlocking { redis.commands.ttl("$redisTableKey:$key").await() }
            if (ttl == -1L) {
                log.warn("Invitation with key [$key] has expired")
                runBlocking { redis.commands.hdel(redisTableKey, key).await() }
            } else {
                log.trace("...invitation [$key] expires in $ttl seconds")
                invitationExpirationJobs[uuid] = ChartedScope.launch {
                    delay(ttl.toDuration(DurationUnit.SECONDS).inWholeMilliseconds)
                    redis.commands.hdel(redisTableKey, key).await()
                }
            }
        }
    }

    override suspend fun createRepositoryMemberInvite(repo: Repository, user: User, to: String): RepoMemberInvite {
        val invite = RepoMemberInvite(repo.id, user.id, UUID.randomUUID())
        redis.commands.hset(
            redisTableKey,
            mapOf(
                invite.id.toString() to json.encodeToString(invite),
            ),
        )

        val fifteenMin = 15.minutes
        redis.commands.set(
            "$redisTableKey:${invite.id}", "nothing important here",
            SetArgs().apply {
                ex(fifteenMin.toJavaDuration())
            },
        ).await()

        invitationExpirationJobs[invite.id] = ChartedScope.launch {
            delay(fifteenMin)
            redis.commands.hdel(redisTableKey, invite.id.toString()).await()
        }

        emails.sendEmail(
            SendEmailRequest.newBuilder().apply {
                this.to = to

                subject = "${if (user.name != null) user.name else "@${user.username}"} has invited you to join the ${repo.name} repository! \uD83C\uDF89"
                context = Struct {
                    putAllFields(
                        mapOf(
                            "repo" to Struct {
                                putAllFields(
                                    mapOf(
                                        "id" to repo.id.toGrpcValue(),
                                        "name" to repo.name.toGrpcValue(),
                                        "description" to repo.description.toGrpcValue(),
                                    ),
                                )
                            }.toGrpcValue(),

                            "user" to Struct {
                                putAllFields(
                                    mapOf(
                                        "id" to user.id.toGrpcValue(),
                                        "name" to user.name.toGrpcValue(),
                                        "username" to user.username.toGrpcValue(),
                                    ),
                                )
                            }.toGrpcValue(),
                        ),
                    )
                }
            }.build(),
        )

        return invite
    }

    override suspend fun acceptRepositoryMemberInvite(id: UUID): RepositoryMember {
        TODO("Not yet implemented")
    }

    override suspend fun declineRepositoryMemberInvite(id: UUID) {
        TODO("Not yet implemented")
    }

    override suspend fun createOrganizationMemberInvite(repoID: Long, userID: Long, to: String): OrgMemberInvite {
        TODO("Not yet implemented")
    }

    override suspend fun acceptOrganizationMemberInvite(id: UUID): RepositoryMember {
        TODO("Not yet implemented")
    }

    override suspend fun declineOrganizationMemberInvite(id: UUID) {
        TODO("Not yet implemented")
    }

    override suspend fun isInviteExpired(id: UUID): Boolean = redis.commands.hexists(redisTableKey, id.toString()).await()
    override fun close() {
        for (job in invitationExpirationJobs.values) job.cancel()
    }
}
