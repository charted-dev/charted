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

import type { ModuleInstall } from '~/env';
import { Stopwatch } from '@noelware/utils';
import router from '~/router';
import App from './App.vue';

import '~/styles/global.css';

const config = useRuntimeConfig();
console.log(` _   _           _     _
| | | | ___  ___| |__ (_)
| |_| |/ _ \\/ __| '_ \\| |
|  _  | (_) \\__ | | | | |
|_| |_|\\___/|___|_| |_|_|

> starting Hoshi v${config.version}+${config.gitCommit}
`);

const app = createApp(App);
const modules = import.meta.glob<boolean, string, { default: ModuleInstall }>('./modules/*.ts');

for (const path in modules) {
    const sw = Stopwatch.createStarted();
    console.log(`[hoshi] INSTALL ${path}`);

    modules[path]().then(({ default: mod }) => {
        mod(app);
        console.log(`[hoshi] INSTALLED ${path} :: ${sw.stop()}`);
    });
}

// @ts-ignore
app.use(router);
router.isReady().then(() => app.mount('#app'));
