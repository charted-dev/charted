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

package org.noelware.charted.cli.commands.services.resolver

import com.github.ajalt.mordant.terminal.Terminal
import org.noelware.charted.cli.logger
import org.noelware.charted.common.Architecture
import org.noelware.charted.common.OperatingSystem
import org.noelware.charted.common.extensions.string.toUri
import java.io.File
import java.net.http.HttpClient
import java.net.http.HttpRequest
import java.net.http.HttpResponse

class EmailServiceResolver(private val terminal: Terminal): BaseServiceResolver {
    private val httpClient = HttpClient.newHttpClient()
    private val downloadUrl = "https://artifacts.noelware.cloud/charted/emails/%s/email-service-%s-%s"
    private val githubDownloadUrl = "https://github.com/charted-dev/email-service/releases/download/%s/email-service-%s-%s"

    override fun resolve(version: String): String {
        val osString = OperatingSystem.current().let { if (it.isMacOS) "darwin" else it.getName() }
        val archString = Architecture.current().let { if (it.isArm64) "arm64" else "amd64" }
        if (File("./bin/email-service-$osString-$archString").exists()) {
            return File("./bin/email-service-$osString-$archString").toString()
        }

        val resolvedDownloadUrl = downloadUrl.format(version, osString, archString)
        terminal.logger.info("Downloading email-service from URL '$resolvedDownloadUrl'")
        val res = httpClient.send(
            HttpRequest.newBuilder().apply {
                GET()
                uri(resolvedDownloadUrl.toUri())
            }.build(),
            HttpResponse.BodyHandlers.ofInputStream(),
        )

        val resp = when (val statusCode = res.statusCode()) {
            500, 503 -> {
                terminal.logger.warn("Noelware's Artifact Registry is currently down [status code $statusCode], falling back to GitHub")
                httpClient.send(
                    HttpRequest.newBuilder().apply {
                        GET()
                        uri(githubDownloadUrl.format(version, osString, archString).toUri())
                    }.build(),
                    HttpResponse.BodyHandlers.ofInputStream(),
                )
            }

            else -> res
        }

        val binDir = File("./bin")
        if (!binDir.exists()) binDir.mkdirs()

        val finalOutput = File(binDir, "email-service-$osString-$archString")
        finalOutput.outputStream().use { stream -> resp.body().transferTo(stream) }

        finalOutput.setExecutable(true)
        return finalOutput.toString()
    }
}
