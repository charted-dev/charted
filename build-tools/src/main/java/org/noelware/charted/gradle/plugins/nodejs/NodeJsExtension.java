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

package org.noelware.charted.gradle.plugins.nodejs;

import org.gradle.api.provider.Property;

public abstract class NodeJsExtension {
    /** Returns the default Node.js version */
    public static String NODEJS_VERSION = "18.9.1";

    /** Returns the default Node.js distributions URL. */
    public static String NODEJS_DIST_URL = "https://nodejs.org/dist";

    /**
     * Returns the Node.js distribution URL to use, if used with a web proxy
     * as an example.
     */
    public abstract Property<String> getNodeDistUrl();

    /**
     * Returns the Node.js version to use.
     */
    public abstract Property<String> getNodeVersion();
}
