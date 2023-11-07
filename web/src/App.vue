<!--
~ 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
~ Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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
import { ToastDescription, ToastProvider, ToastRoot, ToastTitle, ToastViewport } from 'radix-vue';
import { useRandomSplash } from '~/composables/useSplashText';
import ErrorBoundary from './components/ErrorBoundary.vue';
import { isVNode } from 'vue';

const router = useRouter();
const route = router.currentRoute.value.fullPath;
useSeoMeta({
    applicationName: 'Hoshi',
    ogDescription: '🐻‍❄️📦 Free, open source, and reliable Helm Chart registry made in Rust',
    description: '🐻‍❄️📦 Free, open source, and reliable Helm Chart registry made in Rust',
    colorScheme: 'dark light',
    themeColor: '#58548E',
    viewport: 'width=device-width, initial-scale=1',
    ogLocale: 'en_GB',
    ogTitle: 'Hoshi',
    ogUrl: `${import.meta.env.BASE_URL}${route}`
});
</script>

<template>
    <RouterView v-slot="{ Component }">
        <ErrorBoundary>
            <Suspense timeout="0">
                <component :is="Component" />
                <template #fallback>
                    <div class="h-screen justify-center items-center flex flex-col space-y-1.5 container mx-auto">
                        <img
                            alt="Noelware"
                            src="https://cdn.floofy.dev/images/trans.png"
                            draggable="false"
                            class="rounded-lg w-[96px] h-[96px] motion-safe:animate-bounce motion-safe:delay-300"
                        />

                        <span class="font-medium text-lg break-words">{{ useRandomSplash() }}</span>
                    </div>
                </template>
            </Suspense>
        </ErrorBoundary>
    </RouterView>
    <Toaster />
</template>
