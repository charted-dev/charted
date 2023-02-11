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
import software.amazon.awssdk.regions.Region

/**
 * kotlinx.serialization serializer for [Region].
 */
public object AwsRegionSerializer : KSerializer<Region> {
    override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor("aws.Region", PrimitiveKind.STRING)

    override fun serialize(encoder: Encoder, value: Region) {
        encoder.encodeString(value.id())
    }

    override fun deserialize(decoder: Decoder): Region = when (val key = decoder.decodeString()) {
        "af-south-1" -> Region.AF_SOUTH_1
        "ap-east-1" -> Region.AP_EAST_1
        "ap-northeast-1" -> Region.AP_NORTHEAST_1
        "ap-northeast-2" -> Region.AP_NORTHEAST_2
        "ap-northeast-3" -> Region.AP_NORTHEAST_3
        "ap-south-1" -> Region.AP_SOUTH_1
        "ap-southeast-1" -> Region.AP_SOUTHEAST_1
        "ap-southeast-2" -> Region.AP_SOUTHEAST_2
        "ap-southeast-3" -> Region.AP_SOUTHEAST_3
        "aws-cn-global" -> Region.AWS_CN_GLOBAL
        "aws-global" -> Region.AWS_GLOBAL
        "aws-iso-b-global" -> Region.US_ISOB_EAST_1
        "aws-iso-global" -> Region.AWS_ISO_GLOBAL
        "aws-us-gov-global" -> Region.AWS_US_GOV_GLOBAL
        "ca-central-1" -> Region.CA_CENTRAL_1
        "cn-north-1" -> Region.CN_NORTH_1
        "cn-northwest-1" -> Region.CN_NORTHWEST_1
        "eu-central-1" -> Region.EU_CENTRAL_1
        "eu-north-1" -> Region.EU_NORTH_1
        "eu-west-1" -> Region.EU_WEST_1
        "eu-west-2" -> Region.EU_WEST_2
        "eu-west-3" -> Region.EU_WEST_3
        "me-south-1" -> Region.ME_SOUTH_1
        "sa-east-1" -> Region.SA_EAST_1
        "us-east-1" -> Region.US_EAST_1
        "us-east-2" -> Region.US_EAST_2
        "us-gov-east-1" -> Region.US_GOV_EAST_1
        "us-gov-west-1" -> Region.US_GOV_WEST_1
        "us-iso-east-1" -> Region.US_ISO_EAST_1
        "us-iso-west-1" -> Region.US_ISO_WEST_1
        "us-isob-east-1" -> Region.US_ISOB_EAST_1
        "us-west-1" -> Region.US_WEST_1
        "us-west-2" -> Region.US_WEST_2
        else -> error("Unknown region '$key'")
    }
}
