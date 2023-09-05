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
import { User } from '@ncharts/types';

const router = useRouter();
const session = useSessionStore();
const user = ref<User | null>(null);

if (session.available[0]) {
    user.value = await createAuthRequest('/api/users/@me');
}
</script>

<template>
    <nav class="relative">
        <header
            class="flex-row items-center justify-center bg-[#0e0c0e] py-3 lg:pl-2 lg:shadow-md lg:mx-auto lg:flex lg:justify-between"
        >
            <div v-once class="flex mx-auto my-auto">
                <img
                    alt="Noelware"
                    src="https://cdn.floofy.dev/images/trans.png"
                    draggable="false"
                    class="rounded-lg w-[48px] h-[48px]"
                />
            </div>
            <div />
            <div class="mr-4">woof</div>
        </header>
    </nav>
</template>
