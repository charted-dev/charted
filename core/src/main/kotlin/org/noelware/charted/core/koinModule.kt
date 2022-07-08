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

package org.noelware.charted.core

import dev.floofy.haru.Scheduler
import dev.floofy.utils.slf4j.logging
import io.ktor.client.*
import io.ktor.client.engine.okhttp.*
import io.ktor.client.plugins.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.serialization.kotlinx.json.*
import org.apache.commons.validator.routines.EmailValidator
import org.koin.dsl.module
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder

val chartedModule = module {
    single {
        val log by logging("dev.floofy.haru.Scheduler")
        Scheduler {
            handleError { job, t ->
                log.error("Unable to execute job [${job.name}]:", t)
            }
        }
    }

    single {
        EmailValidator.getInstance(true, true)
    }

    single {
        Argon2PasswordEncoder()
    }
}
