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

package org.noelware.charted.testing.yamlTestRunner.directives.internal.statusCode;

import io.ktor.http.HttpStatusCode;
import org.noelware.charted.testing.yamlTestRunner.Descriptor;

/**
 * {@link Descriptor} for the {@link StatusCodeYamlDirective hasStatusCode(int)} directive.
 */
public class StatusCodeDescriptor implements Descriptor {
    private final int httpStatusCode;

    /**
     * Constructs a new instance for this {@link StatusCodeDescriptor}.
     * @param httpStatusCode Status code to resolve
     */
    public StatusCodeDescriptor(int httpStatusCode) {
        this.httpStatusCode = httpStatusCode;
    }

    public HttpStatusCode statusCode() {
        return HttpStatusCode.Companion.fromValue(httpStatusCode);
    }
}
