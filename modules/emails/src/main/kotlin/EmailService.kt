package org.noelware.charted.modules.email

import org.noelware.charted.emails.protobufs.v1.SendEmailRequest
import org.noelware.charted.emails.protobufs.v1.SendEmailResponse
import java.io.Closeable

interface EmailService: Closeable {
    suspend fun sendEmail(request: SendEmailRequest): SendEmailResponse
    suspend fun ping(): Boolean
}
