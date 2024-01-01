<!--
~ 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
~ Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
~
~ Licensed under the Apache License, Version 2.0 (the "License");
~ you may not use this file except in compliance with the License.
~ You may obtain a copy of the License at
~
~    http://www.apache.org/licenses/LICENSE-2.0
~
~ Unless required by applicable law or agreed to in writing, software
~ distributed under the License is distributed on an "AS IS" BASIS,
~ WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
~ See the License for the specific language governing permissions and
~ limitations under the License.
-->

<script setup lang="ts">
import { ApiResponse, responses } from '@ncharts/types';

const router = useRouter();
const route = useRoute();
const store = useSessionStore();
const next = hasOwnProperty(route.query, 'next')
    ? Array.isArray(route.query.next)
        ? route.query.next[0]
        : route.query.next
    : '/';

if (store.isAvailable[0]) {
    await router.push(next as string);
}

const isSignupComputing = ref(false);
const res = await newRequest<ApiResponse<responses.main.Features>>('/api/features', { cache: 'only-if-cached' });
if (!res.success) throw new Error('should always succeed');
</script>

<!--
// if a session is already available, then just go to `next`
if (store.isAvailable[0]) {
    router.push(next as any);
}

const isFormComputing = ref(false);
const res = await newRequest<ApiResponse<responses.main.Features>>('/api/features', {
    cache: 'only-if-cached'
});

if (!res.success) {
    throw new Error('should never happen');
}

const [zodPlugin, submit] = createZodPlugin(
    z.object({
        username: name,
        password: z.string().min(8)
    }),
    async ({ username, password }) => {
        isFormComputing.value = true;
        console.log(`[hoshi:login] logging in as @${username}...`);
        await new Promise((resolve) => setTimeout(resolve, 2000));

        const res = await newRequest<
            ApiResponse<{ session_id: string; refresh_token: string; access_token: string; user_id: number }>
        >('/api/users/login', {
            method: 'post',
            body: {
                username,
                password
            }
        });

        if (!res.success) {
            if (
                res.errors.length === 1 &&
                res.errors[0].code === 'UNKNOWN_USER' &&
                res.errors[0].message.includes('User not found')
            ) {
                // TODO(@auguwu): replace with toast

                console.log(`user @${username} was not found, redirecting to /signup`);
                await router.push('/signup');
            }

            throw new ApiResponseError(res.errors);
        }

        const { access_token, refresh_token, user_id } = res.data;
        console.log(`[hoshi:login] @${username}'s id -> ${user_id}`);

        // get user information
        const res2 = await newRequest<ApiResponse<responses.users.Single>>('/api/users/@me', {
            headers: {
                Authorization: `Bearer ${access_token}`
            }
        });

        if (!res2.success) {
            throw new ApiResponseError(res2.errors);
        }

        store.$patch({
            session: {
                refresh_token,
                access_token,
                user: res2.data
            }
        });

        isFormComputing.value = false;
        await router.push(next as string);
    }
);
</script>

<template>
    <div class="h-screen justify-center items-center flex flex-col space-y-1.5 container mx-auto">
        <img
            alt="Noelware"
            src="https://cdn.floofy.dev/images/trans.png"
            draggable="false"
            class="rounded-lg w-[72px] h-[72px]"
        />

        <h2 class="font-serif font-semibold text-xl">charted-server</h2>
        <h3 class="text-lg">Sign into your account here</h3>
        <div class="md:w-96 w-80 space-x-2 dark:bg-zinc-600/30 rounded-lg shadow-md py-6 px-4">
            <FormKit id="login" type="form" :plugins="[zodPlugin]" :actions="false" @submit="submit">
                <FormKit name="username" type="text" label="Username" />
                <FormKit name="password" type="password" label="Password" autocomplete="current-password" />
                <FormKit
                    id="submit"
                    type="submit"
                    :classes="{
                        input: 'flex w-full justify-center font-mono rounded-md bg-zinc-800 dark:bg-zinc-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:dark:bg-zinc-700 hover:bg-zinc-950 focus-visible:outline focus-visible:outline-offset-2 focus-visible:outline-slate-400'
                    }"
                >
                    Sign In
                </FormKit>
            </FormKit>
        </div>
    </div>
</template>

-->

<template>
    <div
        v-if="!res.data.registrations"
        class="h-screen justify-center items-center flex flex-col space-y-1.5 container mx-auto"
    >
        <img
            alt="Noelware"
            src="https://cdn.floofy.dev/images/trans.png"
            draggable="false"
            class="rounded-lg w-[72px] h-[72px]"
        />

        <h2 class="font-serif font-semibold text-xl">Registrations are disabled!</h2>
        <h3 class="text-lg">
            Please contact the server administrator to create your account as registrations are disabled
        </h3>
    </div>

    <div v-else class="h-screen justify-center items-center flex flex-col space-y-1.5 container mx-auto"></div>
</template>
