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

package org.noelware.charted.common.tests;

import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

import java.io.*;
import java.security.NoSuchAlgorithmException;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;
import org.noelware.charted.common.CryptographyUtils;

@SuppressWarnings("removal")
public class CryptographyUtilsTests {
    @TempDir
    private File tempDir;

    @DisplayName("Test that using a line of text can produce a valid MD5 digest")
    @Test
    public void test_md5Hex() {
        assertThat(CryptographyUtils.md5Hex("Hello, world!")).isNotNull().isEqualTo("6cd3556deb0da54bca060b4c39479839");
    }

    @DisplayName("Test that using a line of text can produce a valid SHA256 digest")
    @Test
    public void test_sha256Hex() {
        assertThat(CryptographyUtils.sha256Hex("beep boop ur gay!"))
                .isNotNull()
                .isEqualTo("da78e20121eb26af9b1e69e15efcc52ca2a8830d5e4aa0781a9c8cee6796236f");
    }

    @DisplayName("Test if checksum hex can be validated on binary data")
    @Test
    public void test_checksumHex() throws IOException, NoSuchAlgorithmException {
        final byte[] bytes = "Hello, world?".getBytes();
        try (final ByteArrayInputStream is = new ByteArrayInputStream(bytes)) {
            assertThat(CryptographyUtils.checksumHex(CryptographyUtils.ALGORITHM_SHA256, is))
                    .isNotNull()
                    .isEqualTo("407e1b6fc892e3340482da07d6c07d8180bdbb1fcf4329ba96559db159316ce7");
        }
    }

    @DisplayName("Check if checksum generation is accurate on files")
    @Test
    public void test_fileChecksum() {
        final File file = new File(tempDir, "uwu.txt");

        // Checks if we can write to the temp directory created by JUnit
        assertThat(file.exists()).isFalse();
        assertThatNoException()
                .isThrownBy(() -> assertThat(file.createNewFile()).isTrue());
        assertThat(file.exists()).isTrue();

        // Now, let's write some mock data
        try (final PrintWriter writer = new PrintWriter(file)) {
            writer.write("We do a little trolling\n");
            writer.printf("I cost %d$ and you cost %d$, we are not the same.", 1_000_000, 10);
        } catch (FileNotFoundException ex) {
            // We shouldn't get this, so fail immediately
            fail(ex);
        }

        try (final FileInputStream fis = new FileInputStream(file)) {
            final String checksum = CryptographyUtils.checksumHex(CryptographyUtils.ALGORITHM_SHA256, fis);
            assertThat(checksum)
                    .isNotNull()
                    .isEqualTo("19ba84e7a737db7d58e67ea9a8b575ab14ff36802f396436a9ef79820a4e6c31");
        } catch (IOException | NoSuchAlgorithmException ex) {
            // We shouldn't get this, so fail immediately
            fail(ex);
        }
    }
}
