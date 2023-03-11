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

package org.noelware.charted.server.bootstrap

import java.io.File

/**
 * Represents a phase when bootstrapping the API server.
 *
 * ## Phases
 * [PreinitPhase] ~> [ConfigureModulesPhase] ~> [StartServerPhase] ~> (killed if exited) [ShutdownPhaseThread]
 */
abstract class BootstrapPhase {
    /**
     * Runs through the bootstrap phase and calls the next one in line.
     */
    abstract suspend fun phaseThrough(config: File)
    companion object {
        /**
         * List of all the [bootstrap phases][BootstrapPhase] available. This should
         * be in chronological order.
         */
        @JvmStatic
        val PHASES: List<BootstrapPhase> = listOf(
            PreinitPhase,
            ConfigureModulesPhase,
            StartServerPhase,
        )
    }
}
