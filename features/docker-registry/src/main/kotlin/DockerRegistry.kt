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

package org.noelware.charted.features.oci.registry

import org.noelware.charted.features.oci.registry.extensions.Extension

/**
 * Represents the implementation details for the home-grown implementation
 * of an OCI registry that plugs into charted-server's authentication mechanisms.
 *
 * This *could* also be used as a Docker Registry if you wish, but it is not
 * recommended.
 */
interface DockerRegistry {
    /**
     * List of extensions that are available
     */
    val extensions: List<Extension>
}
