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

package org.noelware.charted.modules.helm.charts.buildables

import io.ktor.http.content.*
import org.noelware.charted.common.Buildable
import org.noelware.charted.models.repositories.Repository
import kotlin.properties.Delegates

data class UploadReleaseTarball(
    val provenanceFile: PartData.FileItem? = null,
    val tarballFile: PartData.FileItem,
    val version: String,
    val owner: Long,
    val repo: Repository
) {
    class Builder: Buildable<UploadReleaseTarball> {
        var provenanceFile: PartData.FileItem? = null
        var tarballFile: PartData.FileItem by Delegates.notNull()
        var version: String by Delegates.notNull()
        var owner: Long by Delegates.notNull()
        var repo: Repository by Delegates.notNull()

        override fun build(): UploadReleaseTarball = UploadReleaseTarball(
            provenanceFile,
            tarballFile,
            version,
            owner,
            repo,
        )
    }
}
