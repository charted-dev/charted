/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

package org.noelware.charted.testing.framework;

import java.security.KeyStore;
import javax.security.auth.x500.X500Principal;

/**
 * Represents a utility class to generate a temporary SSL certificate for SSL-based connections. This is used
 * in the Elasticsearch test container and in all server tests to test SSL connections.
 */
public class TemporarySSLCertificateGenerator {
    private static final X500Principal LOCALHOST =
            new X500Principal("cn=localhost, OU=charted-server, O=Noelware, C=US");
    private static final X500Principal LOCALHOST_CA =
            new X500Principal("cn=localhostCA, OU=charted-server, O=Noelware, C=US");

    private TemporarySSLCertificateGenerator() {
        /* don't allow direct construction */
    }

    /**
     * Generates a {@link KeyStore} of all the certificates that are available to be used by
     * any resource that requires it. Do note that this is not persistent if <code>file</code>
     * is set to null.
     */
    public static KeyStore generateCertificateKeystore() {
        return null;
    }
}
