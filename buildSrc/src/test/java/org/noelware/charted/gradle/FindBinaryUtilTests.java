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

package org.noelware.charted.gradle;

import static org.junit.jupiter.api.Assertions.*;
import static org.noelware.charted.gradle.util.FileUtil.*;

import java.io.File;
import java.io.IOException;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.junit.jupiter.api.io.TempDir;
import org.noelware.charted.gradle.utils.FindBinaryUtil;
import uk.org.webcompere.systemstubs.environment.EnvironmentVariables;
import uk.org.webcompere.systemstubs.jupiter.SystemStub;
import uk.org.webcompere.systemstubs.jupiter.SystemStubsExtension;
import uk.org.webcompere.systemstubs.properties.SystemProperties;

@ExtendWith(SystemStubsExtension.class)
public class FindBinaryUtilTests {
    @TempDir
    private File tempDir;

    @SystemStub
    private EnvironmentVariables environmentVariables;

    @SystemStub
    private SystemProperties systemProperties;

    @Test
    public void test_FindBinaryUtil_find() throws IOException {
        environmentVariables.set("PATH", tempDir.getAbsolutePath());

        systemProperties.set("os.name", "Windows");
        assertTrue(OperatingSystem.current().isWindows());
        assertNull(FindBinaryUtil.find("awauctl"));

        systemProperties.set("os.name", "Mac OS X");
        assertTrue(OperatingSystem.current().isMacOS());
        assertNull(FindBinaryUtil.find("awauctl"));

        systemProperties.set("os.name", "Linux");
        assertTrue(OperatingSystem.current().isLinux());
        assertNull(FindBinaryUtil.find("awauctl"));

        // Write a shell file to it
        final File awauctl = new File(tempDir, "awauctl");
        writeFile(awauctl, """
        #!/bin/bash

        echo "awauctl [COMMAND] [...OPTIONS]"
        """);

        final String awauPath = FindBinaryUtil.find("awauctl");
        assertNotNull(awauPath);
        assertEquals(awauctl.getAbsolutePath(), awauPath);
    }
}
