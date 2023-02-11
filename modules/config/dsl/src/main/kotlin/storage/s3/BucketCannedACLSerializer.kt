/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.configuration.kotlin.dsl.storage.s3

import kotlinx.serialization.KSerializer
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import software.amazon.awssdk.services.s3.model.BucketCannedACL

public object BucketCannedACLSerializer : KSerializer<BucketCannedACL> {
    override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor("s3.BucketCannedACL", PrimitiveKind.STRING)

    override fun serialize(encoder: Encoder, value: BucketCannedACL) {
        // Since `BucketCannedACL#value` is null, we need to do this
        // ourselves. Thanks, AWS.
        val actualValue = when (value) {
            BucketCannedACL.PUBLIC_READ -> "public-read"
            BucketCannedACL.AUTHENTICATED_READ -> "authenticated-read"
            BucketCannedACL.PRIVATE -> "private"
            BucketCannedACL.PUBLIC_READ_WRITE -> "public-read-write"
            BucketCannedACL.UNKNOWN_TO_SDK_VERSION -> null
            else -> null
        } ?: error("Unknown ACL: $value")

        encoder.encodeString(actualValue)
    }

    override fun deserialize(decoder: Decoder): BucketCannedACL = when (val key = decoder.decodeString()) {
        "private" -> BucketCannedACL.PRIVATE
        "public-read" -> BucketCannedACL.PUBLIC_READ
        "public-read-write" -> BucketCannedACL.PUBLIC_READ_WRITE
        "authenticated-read" -> BucketCannedACL.AUTHENTICATED_READ
        else -> error("Unknown ACL: $key")
    }
}
