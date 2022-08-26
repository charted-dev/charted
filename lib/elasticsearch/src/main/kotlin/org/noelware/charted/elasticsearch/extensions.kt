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

@file:JvmName("ElasticsearchKotlinExtensions")

package org.noelware.charted.elasticsearch

import io.ktor.util.reflect.*
import kotlinx.coroutines.future.await
import org.noelware.charted.common.extensions.unsafeCast
import java.util.concurrent.CompletionStage
import kotlin.reflect.KClass

suspend fun <T, E: Throwable> CompletionStage<T>.awaitOrError(cls: KClass<E>): Pair<T?, E?> = try {
    await() to null
} catch (e: Throwable) {
    if (!e.instanceOf(cls)) {
        throw e
    }

    // TODO: could this be safely casted?
    (null to e).unsafeCast()
}
