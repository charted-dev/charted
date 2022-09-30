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

package org.noelware.charted.testing.kubernetes.minikube.downloader;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.nio.file.Files;
import okhttp3.OkHttpClient;
import okhttp3.Request;
import org.noelware.charted.common.Architecture;
import org.noelware.charted.common.OperatingSystem;
import org.noelware.charted.testing.kubernetes.minikube.downloader.binary.BinaryFileWriter;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class DefaultMinikubeDownloader {
    private static final String MINIKUBE_DOWNLOAD_URL =
            "https://github.com/kubernetes/minikube/releases/download/v%s/minikube-%s-%s%s";
    private static final Logger log = LoggerFactory.getLogger(DefaultMinikubeDownloader.class);
    private static final OkHttpClient client = new OkHttpClient();

    public static String download() throws IOException, InterruptedException {
        final var os = OperatingSystem.current();
        final var arch = Architecture.current();

        // Minikube (at the time that 1.27.0 is released) doesn't support Windows on ARM.
        if (os.isWindows() && arch.isArm()) throw new IOException("Can't install Minikube on Windows ARM systems");

        final var url = MINIKUBE_DOWNLOAD_URL.formatted(
                "1.27.0", os.key(), arch.isX64() ? "amd64" : "arm64", os.isWindows() ? ".exe" : "");
        log.info("Downloading Minikube from URL [{}] with host {} ({})", url, os.key(), arch.getKey());
        final var request = new Request.Builder().url(url).build();

        final var call = client.newCall(request).execute();
        final var body = call.body();
        if (body == null) throw new IOException("Received empty payload!");

        final var header = call.header("Content-Length");
        final var length = header != null ? Double.parseDouble(header) : 1.0;

        // create temporary directory
        final var tmpdir = Files.createTempDirectory("minikube");
        final var file = new File(tmpdir.toFile(), "minikube" + (os.isWindows() ? ".exe" : ""));
        if (!file.exists()) file.createNewFile();
        if (!file.setExecutable(true)) throw new IOException("You do not have permissions to make files executable!");

        final var fos = new FileOutputStream(file);
        try (final var writer = new BinaryFileWriter(fos)) {
            writer.write(body.byteStream(), length);
        } catch (Exception e) {
            throw new RuntimeException(e);
        }

        return file.toString();
    }
}
