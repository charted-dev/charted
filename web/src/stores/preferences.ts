/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
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

/**
 * Represents the preferences that a user can set once they're logged in. This
 * is kept in the browser.
 */
export interface Preferences {
    /**
     * Whether if animations are disabled, by default, this is `'auto'`, which
     * will detect via the browser.
     */
    disableAnimations: boolean | 'auto';

    /**
     * Preferred theme. If `'system'` is used, then it'll use the
     * browser's preferred theme.
     */
    theme: 'dark' | 'light' | 'system';
}
