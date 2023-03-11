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

import static org.junit.jupiter.api.Assertions.*;

import java.io.*;
import java.nio.file.Files;
import java.nio.file.Path;
import java.security.NoSuchAlgorithmException;
import java.util.concurrent.atomic.AtomicReference;
import org.junit.jupiter.api.Test;
import org.noelware.charted.common.CryptographyUtils;

public class CryptographyUtilsTests {
    @Test
    public void test_md5Hex() {
        final var encoded = CryptographyUtils.md5Hex("Hello, world!");
        final var result = "6cd3556deb0da54bca060b4c39479839";

        assertEquals(result, encoded);
    }

    @Test
    public void test_sha256Hex() {
        final var encoded = CryptographyUtils.sha256Hex("beep boop ur gay!");
        final var result = "da78e20121eb26af9b1e69e15efcc52ca2a8830d5e4aa0781a9c8cee6796236f";

        assertEquals(result, encoded);
    }

    @Test
    public void test_checksumHex() throws IOException, NoSuchAlgorithmException {
        final var bytes = "Hello, world?".getBytes();
        try (final var is = new ByteArrayInputStream(bytes)) {
            final var checksum = CryptographyUtils.checksumHex(CryptographyUtils.ALGORITHM_SHA256, is);
            assertEquals("407e1b6fc892e3340482da07d6c07d8180bdbb1fcf4329ba96559db159316ce7", checksum);
        }
    }

    @Test
    public void test_fileChecksum() throws IOException, NoSuchAlgorithmException {
        var tmpdir = new AtomicReference<Path>();
        assertDoesNotThrow(() -> {
            tmpdir.set(Files.createTempDirectory("tmp-" + System.currentTimeMillis()));
        });

        final var td = tmpdir.get();
        final var file = new File(td.toFile(), "uwu.txt");
        assertFalse(file.exists());
        assertDoesNotThrow(() -> {
            assertTrue(file.createNewFile());
        });

        assertTrue(file.exists());
        try (final var writer = new PrintWriter(file)) {
            writer.write("We do a little trolling\n");
            writer.printf("I cost %d$ and you cost %d$, we are not the same.", 1_000_00, 10);
        }

        try (final var is = new FileInputStream(file)) {
            final var checksum = CryptographyUtils.checksumHex(CryptographyUtils.ALGORITHM_SHA256, is);
            assertEquals("c81deaa586399232c1d94ca6e5bfd8e7b1f91755958edd36015dd8ac24c76f4c", checksum);
        }

        // once the tests pass, let's just delete the file
        // and temp directory.
        try {
            Files.delete(file.toPath());
            Files.delete(td);
        } catch (IOException e) {
            System.err.printf("Received exception when cleaning up! Please clean [%s] and [%s] for me!%n", file, td);
        }
    }
}
