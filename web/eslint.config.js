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

import { fileURLToPath } from 'url';

// Node (`ESLINT_FLAT_CONFIG=1 npx eslint`):
//      > import('@augu/eslint-config'):
//      [Module: null prototype] {
//        default: {
//          default: [Getter],
//          javascript: [Getter],
//          perfectionist: [Getter],
//          typescript: [Getter],
//          vue: [Getter]
//        },
//        javascript: [Function: javascript],
//        perfectionist: [AsyncFunction: perfectionist],
//        typescript: [AsyncFunction: typescript],
//        vue: [AsyncFunction: vue]
//      }
//
// Bun:
//     > bun run lint
//     Module {
//       default: [Function: noel],
//       javascript: [Function: javascript],
//       perfectionist: [Function: perfectionist],
//       typescript: [Function: typescript],
//       vue: [Function: vue],
//     }
const noel = await import('@augu/eslint-config').then((mod) =>
    typeof Bun !== 'undefined' ? mod.default : mod.default.default
);

export default noel({
    typescript: {
        tsconfig: fileURLToPath(new URL('.', import.meta.url))
    }
});
