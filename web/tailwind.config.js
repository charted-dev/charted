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

// @ts-check

import defaultConfig from 'tailwindcss/defaultConfig';

/** @param {import('tailwindcss').Config} config */
const defineConfig = (config) => config;

export default defineConfig({
    darkMode: 'class',
    content: ['./**/*.{js,ts,vue,html}'],
    plugins: [require('@tailwindcss/typography'), require('@tailwindcss/forms')],
    theme: {
        extend: {
            fontFamily: {
                // @ts-ignore
                sans: ['Inter', ...defaultConfig.theme.fontFamily.sans],

                // @ts-ignore
                mono: ['"JetBrains Mono"', ...defaultConfig.theme.fontFamily.mono],

                // @ts-ignore
                serif: ['Cantarell', ...defaultConfig.theme.fontFamily.serif]
            }
        }
    }
});
