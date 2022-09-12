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

import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

/**
 * Utilities for using crypto hashing methods in pure Java. This set of static methods
 * replace the ones used in SHAUtils so we don't have unstable API usages.
 *
 * @author Noel
 * @since 10.09.22
 */
public class CryptoUtils {
    private static final String ALGORITHM_MD5 = "MD5";
    private static final String ALGORITHM_SHA1 = "SHA1";
    private static final String ALGORITHM_SHA256 = "SHA-256";

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
    public static String md5(String text) {
        final var digest = getDigest(ALGORITHM_MD5);
        assert digest != null : "Digest was not found."; // this should never happen, but whatever

        digest.update(text.getBytes());
        return bytesToHex(digest.digest());
    }
}

/*
    MessageDigest md = MessageDigest.getInstance("MD5");
    md.update(Files.readAllBytes(Paths.get(filename)));
    byte[] digest = md.digest();
    String myChecksum = DatatypeConverter
      .printHexBinary(digest).toUpperCase();

    assertThat(myChecksum.equals(checksum)).isTrue();

@NotNull
    private static Digest getDigestForName(String algorithm) {
        return switch (algorithm) {
            case "md5" -> new MD5Digest();
            case "sha256" -> new SHA256Digest();
            case "sha512" -> new SHA512Digest();
            default -> throw new IllegalArgumentException("Unknown digest method [%s]".formatted(algorithm));
        };
    }

    @NotNull
    private static String hmac(String algorithm, String data, String key) {
        final var digest = getDigestForName(algorithm);
        final var hmac = new HMac(digest);
        hmac.init(new KeyParameter(key.getBytes()));

        final var in = data.getBytes();
        hmac.update(in, 0, in.length);
        final var out = new byte[hmac.getMacSize()];

        hmac.doFinal(out, 0);
        return bytesToHex(out);
    }

    public String hmacSha256(String data, String key) {
        return hmac("sha256", data, key);
    }

    public String hmacSha512(String data, String key) {
        return hmac("sha512", data, key);
    }

    public String hmacMD5(String data, String key) {
        return hmac("md5", data, key);
    }
 */
