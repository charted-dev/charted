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

<template>
    <ToastRoot
        v-bind="{ ...props, ...useEmitAsProps(emits) }"
        :class="clsx(variants({ type: props.variant }))"
        @update:open="onOpenChange"
    >
        <slot />
    </ToastRoot>
</template>

<script lang="ts">
import type { ToastRootProps } from 'radix-vue';
import type { VariantProps } from './cva';

export interface ToastProps extends ToastRootProps {
    onOpenChange?: (value: boolean) => void;
    variant?: VariantProps['type'];

    // eslint-disable-next-line vue/no-reserved-props
    class?: string;
}
</script>

<script lang="ts" setup>
// eslint-disable-next-line no-duplicate-imports
import { ToastRoot, ToastRootEmits, useEmitAsProps } from 'radix-vue';

// eslint-disable-next-line no-duplicate-imports
import { variants } from './cva';
import { clsx } from '~/utils/clsx';

const props = defineProps<ToastProps>();
const emits = defineEmits<ToastRootEmits>();
</script>
