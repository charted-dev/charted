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

package org.noelware.charted.common;

import java.io.IOException;
import java.io.InputStream;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

/**
 * Utilities for using cryptography hashing methods in pure Java.
 *
 * @author Noel
 * @since 10.09.22
 */
public class CryptographyUtils {
    public static final String ALGORITHM_MD5 = "MD5";
    public static final String ALGORITHM_SHA256 = "SHA-256";

    @NotNull
    private static String bytesToHex(byte[] hex) {
        final var str = new StringBuilder(2 * hex.length);
        for (byte b : hex) {
            final var hexString = Integer.toHexString(0xff & b);
            if (hexString.length() == 1) str.append('0');

            str.append(hexString);
        }

        return str.toString();
    }

    @NotNull
    private static String doHexHash(@NotNull MessageDigest md, byte[] data) {
        md.update(data);
        return bytesToHex(md.digest());
    }

    /**
     * Returns a {@link MessageDigest} of the specified algorithm, or null
     * if it was not found.
     * @param algorithm The algorithm to use.
     */
    @Nullable
    public static MessageDigest getDigest(String algorithm) {
        try {
            return MessageDigest.getInstance(algorithm);
        } catch (NoSuchAlgorithmException e) {
            return null;
        }
    }

    /**
     * Returns the given text as a MD5-encoded string.
     * @param text The text to use to hash.
     */
    @NotNull
    public static String md5Hex(String text) {
        final var digest = getDigest(ALGORITHM_MD5);
        assert digest != null : "Digest was not found."; // this should never happen, but whatever

        digest.update(text.getBytes());
        return bytesToHex(digest.digest());
    }

    public static <I extends InputStream> String checksumHex(String algorithm, I stream)
            throws NoSuchAlgorithmException, IOException {
        final var md = MessageDigest.getInstance(algorithm);
        final var bytes = stream.readAllBytes();
        stream.close();

        return bytesToHex(md.digest(bytes));
    }

    @NotNull
    public static String sha256Hex(String text) {
        final var digest = getDigest(ALGORITHM_SHA256);
        assert digest != null : "Digest was not found.";

        return doHexHash(digest, text.getBytes());
    }
}
