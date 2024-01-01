/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
 * Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
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

/// <reference types="vite-plugin-vue-layouts/client" />
/// <reference types="vite/client" />

/**
 * Runtime configuration that was generated at build-time.
 */
interface RuntimeConfig {
    /** Commit hash from the [canonical Git repository](https://github.com/charted-dev/charted) */
    gitCommit: string;

    /** ISO-8601, RFC3339 formatted date of when the UI was last built */
    buildDate: string;

    /** Version of the web UI and the API server */
    version: string;
}
