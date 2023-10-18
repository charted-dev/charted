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

const SPLASHES = [
    'Things are computing...',
    'is a hotdog a sandwich...?',
    'ERROR 404 - polar bears were not found!',
    'oOoOOOoooOOOo spoopy!!!!',
    'Did you know that we almost went with `helm-server` as the name for charted back in 2022? weird huh...'
] as const;

export const useRandomSplash = () => SPLASHES[Math.floor(Math.random() * SPLASHES.length)];