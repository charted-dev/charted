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
import { fromHighlighter } from 'markdown-it-shikiji/core';
import { getHighlighter } from 'shikiji';
import markdownit from 'markdown-it';

const props = defineProps<{
    content: string;
}>();

const highlighter = await getHighlighter({
    langs: ['console', 'shell', 'yaml'],
    themes: ['rose-pine', 'rose-pine-moon']
});

const md = markdownit().use(
    fromHighlighter(highlighter, {
        themes: {
            light: 'rose-pine-moon',
            dark: 'rose-pine'
        }
    })
);

const rendered = computed(() => {
    console.log(props.content);
    return md.render(props.content);
});
</script>

<template>
    <main class="md" v-html="rendered" />
</template>
