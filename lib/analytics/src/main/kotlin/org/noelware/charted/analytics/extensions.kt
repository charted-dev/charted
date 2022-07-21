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

@file:JvmName("AnalyticsExtensionsKt")

package org.noelware.charted.analytics

import org.noelware.charted.analytics.protobufs.v1.BuildFlavour
import org.noelware.charted.common.DistributionType

fun DistributionType.toBuildFlavour(): BuildFlavour = when (this) {
    DistributionType.DOCKER -> BuildFlavour.DOCKER
    DistributionType.DEB -> BuildFlavour.DEB
    DistributionType.RPM -> BuildFlavour.RPM
    DistributionType.GIT -> BuildFlavour.GIT
    else -> BuildFlavour.UNRECOGNIZED
}
