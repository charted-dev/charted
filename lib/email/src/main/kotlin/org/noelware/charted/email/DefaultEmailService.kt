/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022 Noelware <team@noelware.org>
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

package org.noelware.charted.email

import dev.floofy.utils.slf4j.logging
import org.noelware.charted.common.SetOnceGetValue
import org.noelware.charted.common.data.MailConfig
import java.util.Properties
import javax.mail.Authenticator
import javax.mail.Message
import javax.mail.PasswordAuthentication
import javax.mail.Session
import javax.mail.Transport
import javax.mail.internet.InternetAddress
import javax.mail.internet.MimeMessage

class DefaultEmailService(private val config: MailConfig): EmailService {
    private val _session: SetOnceGetValue<Session> = SetOnceGetValue()
    private val log by logging<DefaultEmailService>()

    init {
        log.info("Initializing session...")

        val auth = config.username != null && config.password != null
        val props = Properties().apply {
            put("mail.smtp.host", config.host)
            put("mail.smtp.port", config.port)
            put("mail.smtp.auth", auth)

            if (config.tls) put("mail.smtp.starttls.enable", "true")
            if (config.ssl) put("mail.smtp.ssl.trust", config.host)
        }

        _session.value = Session.getInstance(
            props,
            object: Authenticator() {
                override fun getPasswordAuthentication(): PasswordAuthentication? = if (auth) PasswordAuthentication(config.username!!, config.password!!) else null
            }
        )
    }

    override fun sendEmail(recipient: String, subject: String, content: String) {
        val message = MimeMessage(_session.value)
        message.setFrom(InternetAddress(config.from))
        message.addRecipient(Message.RecipientType.TO, InternetAddress(recipient))

        message.subject = subject
        message.setText(content)

        Transport.send(message)
    }
}
