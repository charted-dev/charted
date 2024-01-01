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
const store = useSessionStore();
const user = store.session.user!;
</script>

<template>
    <div class="lg:flex lg:flex-col lg:z-50 lg:top-0 lg:bottom-0 lg:fixed lg:w-72 hidden">
        <div class="px-6 dark:bg-zinc-800 overflowy-auto gap-5 flex-col flex-grow flex">
            <div class="mt-4 flex flex-row">
                <img
                    alt="Noelware"
                    src="https://cdn.floofy.dev/images/trans.png"
                    draggable="false"
                    class="rounded-lg w-[40px] h-[40px]"
                />

                <span class="font-mono leading-3 text-2xl my-auto ml-2">charted</span>
            </div>

            <nav class="flex flex-1 flex-col">
                <ul role="list" class="flex flex-col gap-y-7">
                    <li>
                        <ul role="list" class="-mx-2">
                            <li>
                                <RouterLink
                                    to="/"
                                    class="p-2 rounded-md gap-x-2 flex hover:bg-zinc-900 text-zinc-300 hover:text-white"
                                >
                                    <Icon icon="heroicons:home-modern" class="w-6 h-6" />
                                    <span class="leading-3 font-serif font-semibold text-lg my-auto"> Dashboard </span>
                                </RouterLink>
                            </li>
                            <li>
                                <RouterLink
                                    :to="`/~/${user.username}?tab=repositories`"
                                    class="p-2 rounded-md gap-x-2 flex hover:bg-zinc-900 text-zinc-300 hover:text-white"
                                >
                                    <Icon icon="heroicons:folder" class="w-6 h-6" />
                                    <span class="leading-3 font-serif font-semibold text-lg my-auto">
                                        Repositories
                                    </span>
                                </RouterLink>
                            </li>
                            <li>
                                <RouterLink
                                    :to="`/~/${user.username}?tab=organizations`"
                                    class="p-2 rounded-md gap-x-2 flex hover:bg-zinc-900 text-zinc-300 hover:text-white"
                                >
                                    <Icon icon="heroicons:archive-box" class="w-6 h-6" />
                                    <span class="leading-3 font-serif font-semibold text-lg my-auto">
                                        Organizations
                                    </span>
                                </RouterLink>
                            </li>
                            <li v-if="user.admin">
                                <RouterLink
                                    to="/admin"
                                    class="p-2 rounded-md gap-x-2 flex hover:bg-zinc-900 text-zinc-300 hover:text-white"
                                >
                                    <Icon icon="heroicons:beaker" class="w-6 h-6" />
                                    <span class="leading-3 font-serif font-semibold text-lg my-auto">
                                        Admin Portal
                                    </span>
                                </RouterLink>
                            </li>
                        </ul>
                    </li>

                    <li class="mt-auto -mx-2"></li>
                </ul>
            </nav>
        </div>
    </div>
</template>
