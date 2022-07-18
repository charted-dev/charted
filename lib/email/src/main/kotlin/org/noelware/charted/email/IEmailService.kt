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

/**
 * Represents a base email service to send out emails. You can use the [DefaultEmailService] to
 * use the default implementation.
 */
interface IEmailService {
    /**
     * Sends out an email to the [recipient] with the underlying [content].
     * @param recipient The recipient client to send out the email.
     * @param subject The subject of the email
     * @param content The content to send out. This must be an HTML document.
     */
    fun sendEmail(recipient: String, subject: String, content: String)
}
