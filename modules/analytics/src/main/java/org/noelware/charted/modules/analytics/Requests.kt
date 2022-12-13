package org.noelware.charted.modules.analytics

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

class Requests {
    @Serializable
    class InitRequest(val addr: String)
    @Serializable
    class InitResponse(val uuid: String, @SerialName("pub_key") val pubKey: String)
    @Serializable
    class FinalizeRequest(@SerialName("api_token") val apiToken: String)
}