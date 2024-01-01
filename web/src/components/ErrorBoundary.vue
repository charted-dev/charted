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
const error$ = ref<Error | null>(null);
const info$ = ref<string | null>(null);
const hasErr$ = ref(false);
const route = useRoute();

onErrorCaptured((error, _vm, info) => {
    hasErr$.value = true;
    error$.value = error;
    info$.value = info;

    console.error('[hoshi:ui] received error in', info);
    console.error(error);
});
</script>

<template>
    <div v-if="hasErr$" class="h-screen justify-center items-center flex flex-col space-y-2 container mx-auto">
        <img
            alt="Noelware"
            src="https://cdn.floofy.dev/images/trans.png"
            draggable="false"
            class="rounded-lg w-[96px] h-[96px]"
        />

        <h1 class="text-3xl font-serif">Uh oh... an error occured!</h1>
        <h2 class="text-xl">Something went wrong... if this keeps happening, report it to Noelware!</h2>
        <span class="text-lg font-medium">where: {{ info$ }} | route: {{ route.fullPath }}</span>
    </div>

    <slot v-else />
</template>
