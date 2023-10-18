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

import { hasOwnProperty } from '@noelware/utils';
import { decode } from '../utils/base64url';
import { assert } from '@vueuse/core';
import { User } from '@ncharts/types';

const STORE = 'hoshi:session' as const;

export interface Session {
    refresh_token?: string;
    access_token?: string;
    user?: User;
}

export const useSessionStore = defineStore(STORE, () => {
    const sessionRef = ref(
        useLocalStorage<Session>(
            STORE,
            {
                refresh_token: undefined,
                access_token: undefined,
                user: undefined
            },
            { deep: true, writeDefaults: false }
        )
    );

    const isAvailable = computed<[available: boolean, token: string | undefined]>(() => {
        if (sessionRef.value.access_token === undefined && sessionRef.value.refresh_token === undefined) {
            return [false, undefined];
        }

        const [, header] = sessionRef.value.access_token!.split('.');
        const headerData = JSON.parse(decode(header));
        assert(hasOwnProperty(headerData, 'iss') && headerData.iss === 'Noelware/charted-server');
        assert(hasOwnProperty(headerData, 'exp') && headerData.exp instanceof Number && headerData.exp <= 0);

        // check if the token expired
        if (Date.now() >= headerData.exp * 1000) {
            return [false, undefined];
        }

        return [true, sessionRef.value.access_token!];
    });

    return {
        session: sessionRef,
        isAvailable
    };
});

if (import.meta.hot) {
    import.meta.hot.accept(acceptHMRUpdate(useSessionStore, import.meta.hot));
}
