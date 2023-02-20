package org.noelware.charted.modules.email

import org.noelware.charted.emails.protobufs.v1.PingResponseKt
import org.noelware.charted.emails.protobufs.v1.SendEmailRequestKt
import org.noelware.charted.emails.protobufs.v1.SendEmailResponseKt

interface EmailService {
    suspend fun sendEmail(request: SendEmailRequestKt): SendEmailResponseKt
    suspend fun ping(): PingResponseKt
}
