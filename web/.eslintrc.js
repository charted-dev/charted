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

module.exports = {
    extends: ['prettier', '@augu/eslint-config/vue-ts.js'],
    parserOptions: {
        extraFileExtensions: ['.vue'],
        tsconfigRootDir: __dirname,
        project: ['tsconfig.eslint.json']
    },
    rules: {
        'vue/multi-word-component-names': 'off',
        'vue/no-multiple-template-root': 'off', // we're using vue 3, so this doesn't matter
        'vue/max-attributes-per-line': 'off',
        'vue/html-self-closing': 'off',
        'vue/html-indent': ['error', 4]
    }
};
