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

import { plugin, defaultConfig } from '@formkit/vue';
import type { ModuleInstall } from '~/env';

const install: ModuleInstall = (app) => {
    app.use(
        // @ts-ignore
        plugin,
        defaultConfig({
            config: {
                classes: {
                    input: 'block w-full rounded-md border-0 py-1.5 dark:text-white text-gray-900 shadow-sm dark:bg-zinc-700 placeholder:dark:text-gray-400 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-zinc-800/20 sm:text-sm sm:leading-6',
                    label: 'block text-sm font-medium leading-6 text-gray-900 dark:text-white',
                    form: 'space-y-6',
                    message: 'text-red-500 mb-1 text-xs',
                    messages: 'list-none p-0 mt-1 mb-0 mt-0.5',
                    outer: 'mb-4 formkit-disabled:opacity-50',
                    loaderIcon: 'inline-flex items-center w-4 text-gray-600 animate-spin'
                }
            }
        })
    );
};

export default install;
