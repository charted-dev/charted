package org.noelware.charted.modules.email

import org.noelware.charted.emails.protobufs.v1.PingResponseKt
import org.noelware.charted.emails.protobufs.v1.SendEmailRequestKt
import org.noelware.charted.emails.protobufs.v1.SendEmailResponseKt

class DefaultEmailService: EmailService {
    override suspend fun ping(): PingResponseKt {
        TODO("Not yet implemented")
    }

    override suspend fun sendEmail(request: SendEmailRequestKt): SendEmailResponseKt {
        TODO("Not yet implemented")
    }
}
