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

package org.noelware.charted.gradle;

import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.util.HashMap;
import java.util.Locale;
import org.gradle.api.GradleException;
import org.jetbrains.annotations.Nullable;

public class UnixUtils {
    public static @Nullable String getDistroName() throws IOException {
        final var os = OperatingSystem.current();
        if (os.isWindows() || os.isMacOS()) return null;

        final var release = new File("/etc/os-release");
        if (!release.exists()) return null;

        final var map = new HashMap<String, String>();
        try (final var is = new FileInputStream(release)) {
            final var fd = new String(is.readAllBytes());
            final var lines = fd.split("\n");

            for (var line : lines) {
                final var data = line.split("=");
                map.putIfAbsent(data[0].toLowerCase(Locale.ROOT), data[1].replaceAll("\"", ""));
            }
        }

        if (!map.containsKey("name"))
            throw new GradleException("/etc/os-release is corrupted: missing `NAME` attribute.");

        return map.get("name");
    }
}
