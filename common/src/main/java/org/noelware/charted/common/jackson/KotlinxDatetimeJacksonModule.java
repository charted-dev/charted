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

package org.noelware.charted.common.jackson;

import com.fasterxml.jackson.databind.module.SimpleModule;

/**
 * {@link SimpleModule} that adds the {@link LocalDateTimeJsonSerializer} and {@link InstantJsonSerializer} serializers
 * to be easily used with Jackson. Which <code>charted-server</code> uses for Elasticsearch and OpenAPI.
 *
 * @author Noel Towa (cutie@floofy.dev)
 * @since 21.03.23
 */
public class KotlinxDatetimeJacksonModule extends SimpleModule {
    public KotlinxDatetimeJacksonModule() {
        addSerializer(new LocalDateTimeJsonSerializer());
        addSerializer(new InstantJsonSerializer());
    }
}
