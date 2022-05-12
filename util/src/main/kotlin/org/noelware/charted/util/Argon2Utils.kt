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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.util

import org.bouncycastle.crypto.generators.Argon2BytesGenerator
import org.bouncycastle.crypto.params.Argon2Parameters
import java.nio.charset.Charset
import java.security.SecureRandom

private val RANDOM by lazy {
    SecureRandom()
}

fun generateSalt(): ByteArray {
    val salt = ByteArray(16)
    RANDOM.nextBytes(salt)

    return salt
}
fun generatePassword(password: String, salt: ByteArray): String {
    val opsLimit = 4
    val memoryLimit = 1048576
    val outputLen = 32
    val parallelism = 1

    val builder = Argon2Parameters.Builder(Argon2Parameters.ARGON2_id)
        .withVersion(Argon2Parameters.ARGON2_VERSION_13)
        .withIterations(opsLimit)
        .withMemoryAsKB(memoryLimit)
        .withParallelism(parallelism)
        .withSalt(salt)

    val generator = Argon2BytesGenerator()
    generator.init(builder.build())

    val result = ByteArray(outputLen)
    generator.generateBytes(password.toByteArray(Charset.defaultCharset()), result)
    return String(result)
}
