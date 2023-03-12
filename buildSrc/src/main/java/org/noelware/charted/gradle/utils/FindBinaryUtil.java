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

package org.noelware.charted.gradle.utils;

import java.io.File;
import org.noelware.charted.gradle.OperatingSystem;

public class FindBinaryUtil {
    /**
     * Utility to find a binary from the <code>$PATH</code> environment variable. This
     * command has only been tested on Linux, so there is limited support for this.
     *
     * @param binary Binary name to locate
     * @return The full, absolute path to the binary, or <code>null</code> if it couldn't be found.
     */
    public static String find(String binary) {
        // Since this has been tested on Linux, there is limited support
        // for finding a binary.
        if (!OperatingSystem.current().isLinux()) return null;

        final String path = System.getenv("PATH");
        assert path != null : "Unable to locate $PATH environment variable.";

        String[] folders = path.split(":");
        if (folders.length == 0) return null;
        if (folders.length == 1) {
            // Get the first item in the array
            String folder = folders[0];

            // Stat the file to check if it exists
            final File stat = new File(folder);

            // Skip on invalid entries (i.e, files)
            if (!stat.isDirectory()) return null;

            final File[] locatedFiles = stat.listFiles(File::isFile);

            // If an I/O error occurred, then we will have to break,
            // so we don't get anymore or do we just continue?
            if (locatedFiles == null) return null;
            for (File located : locatedFiles) {
                if (located.getName().equalsIgnoreCase(binary)) {
                    return located.getAbsolutePath();
                }
            }
        }

        while (true) {
            folders = ArrayUtil.pop(folders);
            if (folders.length == 0) break;

            // Get the first item in the array
            String folder = folders[0];

            // Stat the file to check if it exists
            final File stat = new File(folder);

            // Skip on invalid entries (i.e, files)
            if (!stat.isDirectory()) continue;

            final File[] locatedFiles = stat.listFiles(File::isFile);

            // If an I/O error occurred, then we will have to break,
            // so we don't get anymore or do we just continue?
            if (locatedFiles == null) break;

            for (File located : locatedFiles) {
                if (located.getName().equalsIgnoreCase(binary)) {
                    return located.getAbsolutePath();
                }
            }
        }

        return null;
    }
}
