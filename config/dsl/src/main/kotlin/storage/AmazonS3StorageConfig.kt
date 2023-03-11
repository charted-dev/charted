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
import org.noelware.charted.common.serializers.SecretStringSerializer
import org.noelware.charted.configuration.kotlin.dsl.storage.s3.AwsRegionSerializer
import org.noelware.charted.configuration.kotlin.dsl.storage.s3.BucketCannedACLSerializer
import org.noelware.charted.configuration.kotlin.dsl.storage.s3.ObjectCannedACLSerializer
import software.amazon.awssdk.regions.Region
import software.amazon.awssdk.services.s3.model.BucketCannedACL
import software.amazon.awssdk.services.s3.model.ObjectCannedACL
import org.noelware.remi.support.s3.AmazonS3StorageConfig as RemiS3StorageConfig

@Serializable
public data class AmazonS3StorageConfig(
    @SerialName("default_object_acl")
    @Serializable(with = ObjectCannedACLSerializer::class)
    val defaultObjectAcl: ObjectCannedACL = ObjectCannedACL.BUCKET_OWNER_FULL_CONTROL,

    @SerialName("default_bucket_acl")
    @Serializable(with = BucketCannedACLSerializer::class)
    val defaultBucketAcl: BucketCannedACL = BucketCannedACL.AUTHENTICATED_READ,

    @SerialName("enable_signer_v4_requests")
    val enableSignerV4Requests: Boolean = false,

    @SerialName("enforce_path_access_style")
    val enforcePathAccessStyle: Boolean = false,

    @SerialName("secret_access_key")
    @Serializable(with = SecretStringSerializer::class)
    val secretAccessKey: String,

    @SerialName("access_key_id")
    val accessKeyId: String,
    val endpoint: String? = null,
    val prefix: String? = null,

    @Serializable(with = AwsRegionSerializer::class)
    val region: Region = Region.US_EAST_1,
    val bucket: String
) {
    internal companion object {
        internal fun fromS3Builder(builder: RemiS3StorageConfig.Builder.() -> Unit = {}): AmazonS3StorageConfig {
            val config = RemiS3StorageConfig.builder().apply(builder).build()
            return AmazonS3StorageConfig(
                config.defaultObjectAcl(),
                config.defaultBucketAcl(),
                config.enableSignerV4Requests(),
                config.enforcePathAccessStyle(),
                config.secretAccessKey(),
                config.accessKeyId(),
                config.endpoint(),
                config.prefix(),
                config.region(),
                config.bucket(),
            )
        }
    }

    public fun toRemiConfig(): RemiS3StorageConfig = RemiS3StorageConfig.builder().apply {
        withEnabledSignerV4Requests(enableSignerV4Requests)
        withEnforcedPathAccessStyle(enforcePathAccessStyle)
        withDefaultObjectAcl(defaultObjectAcl)
        withDefaultBucketAcl(defaultBucketAcl)
        withSecretAccessKey(secretAccessKey)
        withAccessKeyId(accessKeyId)
        withRegion(region)
        withBucket(bucket)

        if (endpoint != null) {
            withEndpoint(endpoint)
        }

        if (prefix != null) {
            withPrefix(prefix)
        }
    }.build()
}
