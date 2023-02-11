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

package org.noelware.charted.modules.email;

import jakarta.mail.MessagingException;

/**
 * Represents the service for sending out emails based off the templates given
 * in the {@code assets/templates} directory in this project.
 */
public interface EmailService {
    /**
     * Sends a test email to the {@code address} specified and any {@code content} to send.
     * @param address The email address to send to
     * @param content The content to send.
     */
    void sendTestEmail(String address, String content) throws MessagingException;
}
