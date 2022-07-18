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

import com.google.common.hash.Hashing;
import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import org.jetbrains.annotations.NotNull;

/**
 * Represents the utilities for hashing.
 *
 * @since 07.07.2022
 * @author Noel <cutie@floofy.dev>
 */
public class SHAUtils {
    private SHAUtils() {}

    @NotNull
    public static String md5(@NotNull String text) {
        return Hashing.md5().hashBytes(text.getBytes(StandardCharsets.UTF_8)).toString();
    }

    @NotNull
    public static <T extends @NotNull InputStream> String sha256Checksum(@NotNull T stream)
            throws IOException {
        var bytes = stream.readAllBytes();
        return Hashing.sha256().hashBytes(bytes).toString();
    }
}
