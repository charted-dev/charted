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

package org.noelware.charted.configuration.kotlin.dsl.storage

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerializationException
import org.noelware.charted.common.serializers.SecretStringSerializer
import java.io.File
import org.noelware.remi.support.gcs.GoogleCloudStorageConfig as RemiGCSConfig

@Serializable
public data class GoogleCloudStorageConfig(
    @SerialName("credentials_file")
    val credentialsFile: String? = null,

    @Serializable(with = SecretStringSerializer::class)
    @SerialName("project_id")
    val projectId: String,
    val bucket: String
) {
    public fun toRemiConfig(): RemiGCSConfig = if (credentialsFile != null) {
        RemiGCSConfig(File(credentialsFile), projectId, bucket)
    } else {
        when {
            System.getProperty("com.google.storage.credentialsFile") != null -> RemiGCSConfig.fromSystemProperty(bucket, projectId, null)
            System.getenv("GOOGLE_APPLICATION_CREDENTIALS") != null -> RemiGCSConfig.fromEnvironmentVariable(bucket, projectId, null)
            else -> throw SerializationException("Unable to determine location of cloud storage credentials file as it is not defined in the configuration object nor in `GOOGLE_CREDENTIALS_FILE` environment variable or in `com.google.storage.credentialsFile` system properties")
        }
    }
}
