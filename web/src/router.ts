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

import { createRouter, createWebHistory } from 'vue-router/auto';
import { setupLayouts } from 'virtual:generated-layouts';
import { hasOwnProperty } from '@noelware/utils';

const router = createRouter({
    history: createWebHistory(),
    extendRoutes(routes) {
        return setupLayouts(routes);
    }
});

router.onError((error, to, from) => {
    console.error(`[hoshi:router] received error while navigation [from ${from.fullPath}] ~> [to ${to.fullPath}]`);
    console.error(error);
});

router.beforeEach((to, _, next) => {
    const needsAuth = to.meta.auth || false;
    const session = useSessionStore();

    if (needsAuth && !session.isAvailable[0]) {
        return next({
            path: '/login',
            query: {
                next: to.fullPath
            }
        });
    }

    return next();
});

export default router;
