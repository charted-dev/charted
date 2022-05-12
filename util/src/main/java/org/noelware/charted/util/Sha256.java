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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.util;

import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import org.jetbrains.annotations.NotNull;

public class Sha256 {
  private static final MessageDigest DIGEST_INSTANCE;

  static {
    try {
      DIGEST_INSTANCE = MessageDigest.getInstance("SHA-256");
    } catch (NoSuchAlgorithmException e) {
      throw new RuntimeException(e);
    }
  }

  private static String bytesToHex(byte[] hash) {
    var builder = new StringBuilder(2 * hash.length);
    for (byte b : hash) {
      var hex = Integer.toHexString(0xff & b);
      if (hex.length() == 1) builder.append('0');
      builder.append(hex);
    }

    return builder.toString();
  }

  public static String encode(@NotNull String data) {
    var hash = DIGEST_INSTANCE.digest(data.getBytes(StandardCharsets.UTF_8));
    return bytesToHex(hash);
  }
}
