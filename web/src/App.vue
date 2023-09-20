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

// catching errors
const error$ = ref<Error | null>(null);
onErrorCaptured((error, instance, info) => {
    console.log(instance);
    error$.value = error;

    console.error('[hoshi:ui] received error on component');
    console.error(error);
    console.error('---');
    console.error(info);
});
</script>

<template>
    <RouterView v-slot="{ Component }">
        <Suspense timeout="0">
            <component :is="Component" />

            <template #fallback>
                <div>Loading...</div>
            </template>
        </Suspense>
    </RouterView>
</template>
