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

import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.JsonObjectBuilder
import kotlinx.serialization.json.buildJsonObject

/**
 * Represents an [exception][RuntimeException] that is used through-out the [DockerRegistry] interface. This
 * is also mapped in the Status Pages Ktor feature to provide similar results to an OCI registry when it fails.
 */
class DockerRegistryException private constructor(
    message: String,
    val code: Code,
    val details: JsonObject? = null
): RuntimeException(message) {
    /**
     * Constructs a new [DockerRegistryException] with a specified [Code] and message
     * to be attached with.
     *
     * @param code The [Code] to attach to this exception
     * @param message A message to use, it will use the [Code.message] property if this is empty
     */
    constructor(code: Code, message: String = ""): this(message.ifEmpty { code.message }, code, null)

    /**
     * Constructs a new [DockerRegistryException] with a specified [Code], message to be attached with,
     * and an [JsonObjectBuilder] DSL builder to attach extra details.
     *
     * @param code The [Code] to attach to this exception
     * @param message A message to use, it will use the [Code.message] property if this is empty
     * @param details DSL object to build a [JsonObject]
     */
    constructor(code: Code, message: String = "", details: JsonObjectBuilder.() -> Unit = {}): this(
        message.ifEmpty { code.message },
        code,
        buildJsonObject(details),
    )

    /**
     * Error code that can be encountered via the implementation
     */
    enum class Code(val message: String) {
        /**
         * This error may be returned when a blob is unknown to the registry in a specified repository. This can be returned
         * with a standard get or if a manifest references an unknown layer during upload.
         */
        UnknownBlob("blob unknown to registry"),

        /**
         * The blob upload encountered an error and can no longer proceed.
         */
        InvalidBlobUpload("blob upload invalid"),

        /**
         * If a blob upload has been cancelled or was never started, this error code may be returned.
         */
        UnknownBlobUpload("blob upload unknown to registry"),

        /**
         * When a blob is uploaded, the registry will check that the content matches the digest provided by the client.
         * The error may include a detail structure with the key “digest”, including the invalid digest string. This error may
         * also be returned when a manifest includes an invalid layer digest.
         */
        InvalidDigest("provided digest did not match uploaded content"),

        /**
         * This error may be returned when a manifest blob is unknown to the registry.
         */
        UnknownBlobManifest("blob unknown to registry"),

        /**
         * During upload, manifests undergo several checks ensuring validity. If those checks fail, this error
         * may be returned, unless a more specific error is included. The detail will contain information
         * the failed validation.
         */
        InvalidManifest("manifest invalid"),

        /**
         * This error is returned when the manifest, identified by name and tag is unknown to the repository.
         */
        UnknownManifest("manifest unknown"),

        /**
         * During manifest upload, if the manifest fails signature verification, this error will be returned.
         */
        UnverifiedManifest("manifest failed signature verification"),

        /**
         * Invalid repository name encountered either during manifest validation or any API operation.
         */
        InvalidName("invalid repository name"),

        /**
         * This is returned if the name used during an operation is unknown to the registry.
         */
        UnknownName("repository name not known to registry"),

        /**
         * Returned when the “n” parameter (number of results to return) is not an integer, or “n” is negative.
         */
        InvalidPaginationNumber("invalid number of results requested"),

        /**
         * When a layer is uploaded, the provided range is checked against the uploaded chunk. This error is returned
         * if the range is out of order.
         */
        InvalidRange("invalid content range"),

        /**
         * When a layer is uploaded, the provided size will be checked against the uploaded content.
         * If they do not match, this error will be returned.
         */
        InvalidSize("provided length did not match content length"),

        /**
         * During a manifest upload, if the tag in the manifest does not match the uri tag,
         * this error will be returned.
         */
        InvalidTag("manifest tag did not match URI"),

        /**
         * authentication required	The access controller was unable to authenticate the client. Often this
         * will be accompanied by a Www-Authenticate HTTP response header indicating how to authenticate.
         */
        Unauthorized("authentication required"),

        /**
         * The access controller denied access for the operation on a resource.
         */
        Denied("requested access to this resource is denied"),

        /**
         * The operation was unsupported due to a missing implementation or invalid set of parameters.
         */
        Unsupported("The operation is unsupported.")
    }
}
