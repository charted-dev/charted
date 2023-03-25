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

package org.noelware.charted.server.util

import org.koin.core.module.Module

/**
 * Composes a list of modules into one big list of modules for Koin.
 * @param module The module to add to the list (this is required)
 * @param modules Other modules to combine
 * @return combined list of modules
 */
fun composeKoinModules(module: Module, vararg modules: Module): List<Module> {
    val combined = mutableListOf(module)
    for (mod in modules) combined.add(mod)

    return combined
}

fun composeKoinModules(modules: List<Module>, vararg mods: Module): List<Module> = modules + mods.toList()
