/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright (c) 2022-2023 Noelware, LLC. <team@noelware.org>
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

package org.noelware.charted.modules.email;

import jakarta.mail.*;
import jakarta.mail.internet.InternetAddress;
import jakarta.mail.internet.MimeMessage;
import java.util.Date;
import java.util.Properties;
import org.noelware.charted.configuration.kotlin.dsl.SMTPConfig;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class DefaultEmailService implements EmailService {
    private final Logger LOG = LoggerFactory.getLogger(DefaultEmailService.class);
    private final SMTPConfig config;

    public DefaultEmailService(SMTPConfig config) {
        this.config = config;
    }

    /**
     * Sends a test email to the {@code address} specified and any {@code content} to send.
     * @param address The email address to send to
     * @param content The content to send.
     */
    @Override
    public void sendTestEmail(String address, String content) throws MessagingException {
        final Session session = createSession();

        LOG.info("Sending email with content [{}] to address [{}]", content, address);
        final Message message = new MimeMessage(session);
        message.setFrom(new InternetAddress(config.getFrom()));
        message.setRecipient(Message.RecipientType.TO, new InternetAddress(address));
        message.setSubject("This is a test email from charted-server!");
        message.setSentDate(new Date());
        message.setText(content);

        Transport.send(message, message.getAllRecipients());
    }

    private Session createSession() {
        LOG.info("Creating SMTP session...");

        final Properties props = new Properties();
        props.put("mail.smtp.host", config.getHost());
        props.put("mail.smtp.port", config.getPort());

        if (config.getTls()) {
            props.put("mail.smtp.starttls.enable", "true");
        }

        if (config.getSsl()) {
            props.put("mail.smtp.ssl.trust", config.getHost());
        }

        if (config.getUsername() != null) {
            if (config.getPassword() == null) {
                throw new IllegalStateException("Missing `smtp.password` configuration key with `smtp.username`");
            }

            props.put("mail.smtp.auth", true);
        }

        return Session.getInstance(
                props,
                config.getUsername() == null
                        ? null
                        : new Authenticator() {
                            @Override
                            protected PasswordAuthentication getPasswordAuthentication() {
                                return new PasswordAuthentication(config.getUsername(), config.getPassword());
                            }
                        });
    }
}
