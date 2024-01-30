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
// toggles for dropdown menus
const isUserListOpen = ref(false);

const user = { username: 'noel', admin: true };
const { version, gitCommit } = useRuntimeConfig();
</script>

<template>
    <div class="lg:flex lg:flex-col lg:z-50 lg:top-0 lg:bottom-0 lg:fixed lg:w-72 hidden">
        <div class="px-6 dark:bg-[#0a080a] overflow-y-auto gap-5 flex-col flex-grow flex">
            <div class="mt-4 flex flex-row justify-center">
                <img
                    alt="Noelware, LLC."
                    src="https://cdn.floofy.dev/images/trans.png"
                    draggable="false"
                    class="rounded-lg w-[32px] h-[32px]"
                />

                <div class="flex-grow" />
                <div class="grid grid-cols-3 gap-2">
                    <span>hewo</span>
                    <span>hewo</span>
                    <DropdownMenuRoot v-model:open="isUserListOpen">
                        <DropdownMenuTrigger
                            class="rounded-lg w-[32px] h-[32px] inline-flex items-center justify-center"
                        >
                            <img
                                alt="@noel's avatar"
                                src="https://cdn.floofy.dev/images/August.png"
                                draggable="false"
                                class="rounded-lg w-[32px] h-[32px]"
                            />
                        </DropdownMenuTrigger>
                        <DropdownMenuPortal>
                            <DropdownMenuContent
                                class="min-w-[220px] outline-none bg-gray-950 rounded-md p-[5px] shadow-[0px_10px_38px_-10px_rgba(22,_23,_24,_0.35),_0px_10px_20px_-15px_rgba(22,_23,_24,_0.2)] will-change-[opacity,transform] data-[side=top]:animate-slideDownAndFade data-[side=right]:animate-slideLeftAndFade data-[side=bottom]:animate-slideUpAndFade data-[side=left]:animate-slideRightAndFade"
                            >
                                weow
                            </DropdownMenuContent>
                        </DropdownMenuPortal>
                    </DropdownMenuRoot>
                </div>
            </div>
            <nav class="flex flex-1 flex-col">
                <ul role="list" class="relative flex flex-col gap-y-7">
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
                                    :to="`/~/${user.username}?tab=organizations`"
                                    class="p-2 rounded-md gap-x-2 flex hover:bg-zinc-900 text-zinc-300 hover:text-white"
                                >
                                    <Icon icon="heroicons:archive-box" class="w-6 h-6" />
                                    <span class="leading-3 font-serif font-semibold text-lg my-auto">
                                        Organizations
                                    </span>
                                </RouterLink>
                            </li>
                            <li>
                                <RouterLink
                                    :to="`/@me/settings`"
                                    class="p-2 rounded-md gap-x-2 flex hover:bg-zinc-900 text-zinc-300 hover:text-white"
                                >
                                    <Icon icon="material-symbols:settings-suggest-rounded" class="w-6 h-6" />
                                    <span class="leading-3 font-serif font-semibold text-lg my-auto"> Settings </span>
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
                </ul>
            </nav>
            <div class="relative bottom-0 h-12">
                <a
                    :href="`https://github.com/charted-dev/charted/releases/${version}`"
                    target="_blank"
                    class="font-mono hover:underline"
                >
                    v{{ version }}+{{ gitCommit }}
                </a>
            </div>
        </div>
    </div>
</template>
