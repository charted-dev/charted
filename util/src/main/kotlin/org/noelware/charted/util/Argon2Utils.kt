package org.noelware.charted.util

import de.mkammerer.argon2.Argon2Constants
import java.security.SecureRandom

private val RANDOM by lazy {
    SecureRandom()
}

fun generateSalt(): ByteArray {
    val salt = ByteArray(16)
    RANDOM.nextBytes(salt)

    return salt
}

// TODO: use https://docs.spring.io/spring-security/site/docs/5.2.0.RELEASE/reference/htmlsingle/#pe-a2pe
fun generatePassword(password: String, salt: ByteArray): String = ""
