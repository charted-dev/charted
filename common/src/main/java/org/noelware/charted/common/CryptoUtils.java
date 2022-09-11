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

import org.bouncycastle.crypto.Digest;
import org.bouncycastle.crypto.digests.MD5Digest;
import org.bouncycastle.crypto.digests.SHA256Digest;
import org.bouncycastle.crypto.digests.SHA512Digest;
import org.bouncycastle.crypto.macs.HMac;
import org.bouncycastle.crypto.params.KeyParameter;
import org.jetbrains.annotations.NotNull;

/**
 * Utilities for using crypto hashing methods in pure Java. This set of static methods
 * replace the ones used in {@link SHAUtils} so we don't have unstable API usages.
 *
 * @author Noel
 * @since 10.09.22
 */
public class CryptoUtils {
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
}
