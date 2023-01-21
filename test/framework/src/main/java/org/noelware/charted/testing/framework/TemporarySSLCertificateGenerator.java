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

import java.math.BigInteger;
import java.security.KeyPair;
import java.security.KeyPairGenerator;
import java.security.NoSuchAlgorithmException;
import java.security.cert.CertificateException;
import java.security.cert.X509Certificate;
import java.time.Instant;
import java.util.Date;
import java.util.concurrent.TimeUnit;
import org.bouncycastle.asn1.oiw.OIWObjectIdentifiers;
import org.bouncycastle.asn1.x500.X500Name;
import org.bouncycastle.asn1.x509.AlgorithmIdentifier;
import org.bouncycastle.asn1.x509.BasicConstraints;
import org.bouncycastle.asn1.x509.Extension;
import org.bouncycastle.asn1.x509.SubjectPublicKeyInfo;
import org.bouncycastle.cert.CertIOException;
import org.bouncycastle.cert.X509ExtensionUtils;
import org.bouncycastle.cert.X509v3CertificateBuilder;
import org.bouncycastle.cert.jcajce.JcaX509CertificateConverter;
import org.bouncycastle.cert.jcajce.JcaX509v3CertificateBuilder;
import org.bouncycastle.jce.provider.BouncyCastleProvider;
import org.bouncycastle.operator.ContentSigner;
import org.bouncycastle.operator.DigestCalculator;
import org.bouncycastle.operator.OperatorCreationException;
import org.bouncycastle.operator.bc.BcDigestCalculatorProvider;
import org.bouncycastle.operator.jcajce.JcaContentSignerBuilder;

/**
 * Represents a utility class to generate a temporary SSL certificate for SSL-based connections. This is used
 * in the Elasticsearch test container and in all server tests to test SSL connections.
 */
public class TemporarySSLCertificateGenerator {
    private static final X500Name LOCALHOST = new X500Name("cn=localhost, OU=charted-server, O=Noelware, C=US");

    private TemporarySSLCertificateGenerator() {
        /* don't allow direct construction */
    }

    /**
     * Generates a new {@link X509Certificate} of a self-signed certificate that lasts for 15 minutes, this is only
     * made for unit tests and nothing else.
     */
    public static X509Certificate generateCertificate()
            throws NoSuchAlgorithmException, OperatorCreationException, CertIOException, CertificateException {
        // We need to generate a random KeyPair
        final KeyPairGenerator keyPairGenerator = KeyPairGenerator.getInstance("RSA");
        keyPairGenerator.initialize(4096);
        final KeyPair keyPair = keyPairGenerator.generateKeyPair();

        final Instant NOW = Instant.now();

        // Since our unit/integration tests last ~15 minutes (depending on the operating system), we will
        // set the end now + 15 minutes
        final Instant END = NOW.plus(15, TimeUnit.MINUTES.toChronoUnit());

        // Now, we need to get a Date of <=before >=after
        final Date notBefore = Date.from(NOW);
        final Date notAfter = Date.from(END);
        final ContentSigner signer = new JcaContentSignerBuilder("SHA256withRSA").build(keyPair.getPrivate());

        final X509v3CertificateBuilder builder = new JcaX509v3CertificateBuilder(
                LOCALHOST, BigInteger.valueOf(NOW.toEpochMilli()),
                notBefore, notAfter,
                LOCALHOST, keyPair.getPublic());

        final SubjectPublicKeyInfo publicKeyInfo =
                SubjectPublicKeyInfo.getInstance(keyPair.getPublic().getEncoded());

        final DigestCalculator digCalc =
                new BcDigestCalculatorProvider().get(new AlgorithmIdentifier(OIWObjectIdentifiers.idSHA1));

        builder.addExtension(
                Extension.subjectKeyIdentifier,
                false,
                new X509ExtensionUtils(digCalc).createSubjectKeyIdentifier(publicKeyInfo));

        builder.addExtension(
                Extension.authorityKeyIdentifier,
                false,
                new X509ExtensionUtils(digCalc).createAuthorityKeyIdentifier(publicKeyInfo));

        builder.addExtension(Extension.basicConstraints, true, new BasicConstraints(true));
        return new JcaX509CertificateConverter()
                .setProvider(new BouncyCastleProvider())
                .getCertificate(builder.build(signer));
    }
}
