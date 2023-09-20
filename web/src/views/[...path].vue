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
const route = useRoute();
</script>

<template>
    <div class="relative">
        <div class="h-screen flex flex-col mx-auto container justify-center items-center space-y-3">
            <img
                src="https://cdn.floofy.dev/images/trans.png"
                draggable="false"
                class="w-[145px] h-[146px] rounded-lg"
                alt="noelware"
            />

            <h1 class="font-semibold font-serif text-4xl">Page Not Found</h1>
            <h2 class="text-2xl">
                Page <code>{{ route.fullPath }}</code> was not found.
            </h2>
        </div>
    </div>
</template>
