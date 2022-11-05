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

@file:JvmName("RegistryConstantsKt")

package org.noelware.charted.modules.docker.registry

/** Regular expression for an OCI containers' repository name */
val REPOSITORY_NAME_REGEX: Regex = "[a-z0-9]+([._-][a-z0-9]+)*(/[a-z0-9]+([._-][a-z0-9]+)*)*".toRegex()

/** Regular expression for an OCI container tag */
val TAG_REGEX: Regex = "[a-zA-Z0-9_][a-zA-Z0-9._-]{0,127}".toRegex()
