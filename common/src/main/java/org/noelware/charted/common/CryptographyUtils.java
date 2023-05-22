/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.common;

import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.Objects;
import org.apache.commons.compress.utils.IOUtils;
import org.bouncycastle.util.encoders.Hex;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

/**
 * Utilities for using cryptography hashing methods in pure Java.
 *
 * @author Noel Towa (cutie@floofy.dev)
 * @since 10.09.22
 */
public class CryptographyUtils {
    /**
     * Constant for the SHA256 algorithm to be used with
     * {@link MessageDigest}.
     */
    public static final String ALGORITHM_SHA256 = "SHA-256";

    /**
     * Constant for the MD5 algorithm to be used with
     * {@link MessageDigest}.
     */
    public static final String ALGORITHM_MD5 = "MD5";

    private CryptographyUtils() {
        throw new IllegalArgumentException("You are not allowed to construct CryptographyUtils");
    }

    /**
     * @param algorithm Algorithm that is valid by {@link MessageDigest#getInstance(String)}
     * @return {@link MessageDigest} or <code>null</code> if not found.
     */
    @Nullable
    public static MessageDigest getMessageDigest(String algorithm) {
        try {
            return MessageDigest.getInstance(algorithm);
        } catch (NoSuchAlgorithmException ignored) {
            return null;
        }
    }

    /**
     * Returns the given text as an MD5-encoded string.
     * @param text Text to encode
     */
    @NotNull
    public static String md5(String text) {
        final MessageDigest digest = getMessageDigest(ALGORITHM_MD5);
        if (digest == null) {
            throw new IllegalStateException(
                    "This is an internal bug. Please report this: https://githubn.com/charted-dev/charted/issues/new");
        }

        digest.update(text.getBytes(StandardCharsets.UTF_8));
        return new String(Hex.encode(digest.digest()));
    }

    @NotNull
    public static String checksumHex(String algorithm, InputStream stream)
            throws NoSuchAlgorithmException, IOException {
        final MessageDigest digest = MessageDigest.getInstance(algorithm);
        if (stream.available() == 0) {
            throw new IOException("Cannot use 0-length stream for checksum");
        }

        final byte[] data = IOUtils.toByteArray(stream);
        return new String(Hex.encode(digest.digest(data)));
    }

    @Deprecated(forRemoval = true, since = "0.4.0-unstable.4")
    @NotNull
    public static String sha256Stream(InputStream stream) throws IOException {
        return sha256(stream);
    }

    @Deprecated(forRemoval = true, since = "0.4.0-unstable.4")
    @NotNull
    public static String sha256Hex(String text) {
        return sha256(text);
    }

    @Deprecated(forRemoval = true, since = "0.4.0-unstable.4")
    @NotNull
    public static String md5Hex(String text) {
        return md5(text);
    }

    @NotNull
    public static String sha256(@NotNull InputStream stream) throws IOException {
        Objects.requireNonNull(stream, "Stream cannot be null");
        if (stream.available() <= 0) {
            throw new IOException("Cannot use under 0-length stream for checksum");
        }

        final MessageDigest digest = getMessageDigest(ALGORITHM_SHA256);
        if (digest == null) {
            throw new IllegalStateException();
        }

        final byte[] data = IOUtils.toByteArray(stream);
        digest.update(data);

        return new String(Hex.encode(digest.digest()));
    }

    @NotNull
    public static String sha256(String text) {
        // Return an empty string on null or if it is empty.
        if (text == null || text.isEmpty()) return "";

        final MessageDigest digest = getMessageDigest(ALGORITHM_SHA256);
        if (digest == null) {
            throw new IllegalStateException();
        }

        digest.update(text.getBytes(StandardCharsets.UTF_8));
        return new String(Hex.encode(digest.digest()));
    }
}
