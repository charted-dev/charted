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

import { ofetch, FetchOptions } from 'ofetch';
import { useSessionStore } from '../stores/session';
import { hasOwnProperty } from '@noelware/utils';

const _clientRef = ref<ReturnType<(typeof ofetch)['create']>>(
    ofetch.create({
        ignoreResponseError: true, // we will handle it on our own
        retry: 3,
        timeout: 5000, // timeout after 5 seconds
        responseType: 'json',
        headers: {
            'Content-Type': 'application/json'
        }
    })
);

/**
 * Composable to return an ofetch instance that resolves to the
 * options that Hoshi needs.
 */
export const useFetch = () => _clientRef.value;
export const newRequest = <T = any, RT extends NonNullable<FetchOptions['responseType']> = 'json'>(
    ...args: Parameters<typeof ofetch>
) => {
    const fetch = useFetch();
    const options = args.length === 1 ? {} : args[1]!;
    const headers = new Headers(options.headers || {});
    const store = useSessionStore();

    if (store.isAvailable[0]) {
        headers.append('Authorization', `Bearer ${store.isAvailable[1]}`);
    }

    return fetch<T, RT>(args[0], {
        headers,
        ...options
    } as unknown as FetchOptions<RT>);
};
