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

import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializer
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import software.amazon.awssdk.services.s3.model.ObjectCannedACL

@OptIn(ExperimentalSerializationApi::class)
@Serializer(forClass = ObjectCannedACL::class)
public object ObjectCannedACLSerializer : KSerializer<ObjectCannedACL> {
    override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor("aws.s3.ObjectCannedACL", PrimitiveKind.STRING)

    override fun serialize(encoder: Encoder, value: ObjectCannedACL) {
        val aclType = when (value) {
            ObjectCannedACL.PRIVATE -> "private"
            ObjectCannedACL.PUBLIC_READ -> "public-read"
            ObjectCannedACL.PUBLIC_READ_WRITE -> "public-read-write"
            ObjectCannedACL.AUTHENTICATED_READ -> "authenticated-read"
            ObjectCannedACL.AWS_EXEC_READ -> "aws-exec-read"
            ObjectCannedACL.BUCKET_OWNER_READ -> "bucket-owner-read"
            ObjectCannedACL.BUCKET_OWNER_FULL_CONTROL -> "bucket-owner-full-control"
            ObjectCannedACL.UNKNOWN_TO_SDK_VERSION -> error("cannot use `UNKNOWN_TO_SDK_VERSION`.")
            else -> error("Unknown object ACL: $value")
        }

        encoder.encodeString(aclType)
    }

    override fun deserialize(decoder: Decoder): ObjectCannedACL = when (val key = decoder.decodeString()) {
        "bucket-owner-full-control" -> ObjectCannedACL.BUCKET_OWNER_FULL_CONTROL
        "bucket-owner-read" -> ObjectCannedACL.BUCKET_OWNER_FULL_CONTROL
        "authenticated-read" -> ObjectCannedACL.AUTHENTICATED_READ
        "aws-exec-read" -> ObjectCannedACL.AWS_EXEC_READ
        "public-read-write" -> ObjectCannedACL.PUBLIC_READ_WRITE
        "public-read" -> ObjectCannedACL.PUBLIC_READ
        "private" -> ObjectCannedACL.PRIVATE
        else -> error("Unknown ACL type: $key")
    }
}
