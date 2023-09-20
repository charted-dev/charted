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

import { createPinia } from 'pinia';
import { createHead } from '@vueuse/head';
import { createApp } from 'vue';
import router from './router';
import App from './App.vue';

import './styles/global.css';

const pinia = createPinia();
const app = createApp(App);

// TODO(@auguwu): allow sending it to Sentry from `sentry_dsn` in server
// configuration
app.config.errorHandler = (error, _vm, info) => {
    console.error('[hoshi:ui] received error');
    console.error(error);
    console.error('~~~');
    console.error(info);
};

app.use(
    createHead({
        title: 'Hoshi',
        link: [
            // TODO(@auguwu): switch to charted branding
            { rel: 'shortcut icon', href: 'https://cdn.floofy.dev/images/trans.png' }
        ]
    })
);

app.use(pinia);
app.use(router);

router.isReady().then(() => {
    app.mount('#app');
});
