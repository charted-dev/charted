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

package org.noelware.charted.database.controllers

import dev.floofy.utils.exposed.asyncTransaction
import dev.samstevens.totp.code.*
import dev.samstevens.totp.qr.QrData
import dev.samstevens.totp.qr.ZxingPngQrGenerator
import dev.samstevens.totp.recovery.RecoveryCodeGenerator
import dev.samstevens.totp.secret.DefaultSecretGenerator
import dev.samstevens.totp.secret.SecretGenerator
import dev.samstevens.totp.time.SystemTimeProvider
import dev.samstevens.totp.time.TimeProvider
import io.ktor.http.*
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.buildJsonObject
import org.jetbrains.exposed.sql.deleteWhere
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.Snowflake
import org.noelware.charted.database.entities.User2faEntity
import org.noelware.charted.database.entities.UserEntity
import org.noelware.charted.database.tables.User2faTable
import org.noelware.charted.database.tables.UserTable

object User2faController {
    private val secretGenerator: SecretGenerator = DefaultSecretGenerator(64)
    private val timeProvider: TimeProvider = SystemTimeProvider()
    private val codeGenerator: CodeGenerator = DefaultCodeGenerator()
    private val codeVerifier: CodeVerifier = DefaultCodeVerifier(codeGenerator, timeProvider)

    suspend fun enabled(id: Long): Boolean = asyncTransaction(ChartedScope) {
        User2faEntity.find { User2faTable.account eq id }.firstOrNull() != null
    }

    suspend fun qrCode(id: Long): Pair<String, ByteArray>? {
        if (!enabled(id)) return null
        val d = asyncTransaction(ChartedScope) {
            User2faEntity.find { User2faTable.account eq id }.first()
        }

        val data = QrData.Builder().apply {
            label("charted-server")
            secret(d.secret)
            issuer("Noelware/charted-server")
            algorithm(HashingAlgorithm.SHA1)
            digits(6)
            period(30)
        }.build()

        val qrGenerator = ZxingPngQrGenerator()
        return qrGenerator.imageMimeType to qrGenerator.generate(data)
    }

    suspend fun enable2fa(id: Long): Pair<HttpStatusCode, JsonObject> {
        val secret = secretGenerator.generate()
        val recoveryCodeGenerator = RecoveryCodeGenerator()
        val codes = recoveryCodeGenerator.generateCodes(16)
        val twoFactorAuthId = Snowflake.generate()

        asyncTransaction(ChartedScope) {
            User2faEntity.new(twoFactorAuthId) {
                this.secret = secret
                this.account = UserEntity.find { UserTable.id eq id }.first()
                this.recoveryKeys = codes
            }
        }

        return HttpStatusCode.OK to buildJsonObject {
            put("recovery_codes", JsonArray(codes.toList().map { JsonPrimitive(id) }))
        }
    }

    suspend fun disable2fa(id: Long) {
        asyncTransaction(ChartedScope) {
            User2faTable.deleteWhere { User2faTable.id eq id }
        }
    }

    suspend fun verify(id: Long, code: String): Boolean {
        if (!enabled(id)) return false
        val twoFactorAuth = asyncTransaction(ChartedScope) {
            User2faEntity.find { User2faTable.account eq id }.first()
        }

        return codeVerifier.isValidCode(twoFactorAuth.secret, code)
    }
}
