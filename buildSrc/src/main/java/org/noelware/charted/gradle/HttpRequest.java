/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

import com.google.gson.Gson;
import com.google.gson.JsonObject;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpResponse;

public class HttpRequest {
    private static final HttpClient httpClient = HttpClient.newHttpClient();
    private static final Gson gson = new Gson();

    public static InputStream request(String url) throws IOException, InterruptedException {
        return request(URI.create(url));
    }

    public static InputStream request(URI url) throws IOException, InterruptedException {
        final var request = java.net.http.HttpRequest.newBuilder(url)
                .GET()
                .setHeader("User-Agent", "Noelware/charted-server")
                .build();

        final var response = httpClient.send(request, HttpResponse.BodyHandlers.ofInputStream());
        return response.body();
    }

    public static JsonObject json(String url) throws IOException, InterruptedException {
        return json(URI.create(url));
    }

    public static JsonObject json(URI url) throws IOException, InterruptedException {
        final var response = request(url);
        try (final var reader = new InputStreamReader(response)) {
            return gson.fromJson(reader, JsonObject.class);
        }
    }

    public static String text(String url) throws IOException, InterruptedException {
        return text(URI.create(url));
    }

    public static String text(URI url) throws IOException, InterruptedException {
        try (final var is = request(url)) {
            final var bytes = is.readAllBytes();
            return new String(bytes);
        }
    }
}
