/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
 * Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import { cva } from 'class-variance-authority';

const headingClass = cva('dark:text-white text-[#15151A]', {
    variants: {
        level: {
            1: 'font-serif font-bold text-5xl tracking-tight scroll-m-20',
            2: 'font-serif font-bold text-4xl scroll-m-20 tracking-tight first:mt-0',
            3: 'font-serif font-semibold text-2xl scroll-m-20 tracking-tight',
            4: 'text-xl scroll-m-20 tracking-tight',
            5: 'text-lg',
            6: 'text-sm'
        }
    }
});

const createHeadingComponent = (level: 1 | 2 | 3 | 4 | 5 | 6) =>
    defineComponent({
        name: `Heading${level}`,
        setup(props, { slots }) {
            return () =>
                h(
                    `h${level}`,
                    {
                        class: headingClass({ level }),
                        ...props
                    },
                    slots.default!()
                );
        }
    });

/**
 * `<H1>{...content}</H1>`
 *
 * Hoshi's semantics of a heading level 1, i.e, the `<h1>` component.
 *
 * @example
 * ```vue
 * <script setup lang="ts">
 * import { H1 } from '~/components/ui/typography';
 * </script>
 * <template>
 *     <H1>Hello, world!</H1>
 * </template>
 * ```
 */
export const H1 = createHeadingComponent(1);

/**
 * `<H2>{...content}</H2>`
 *
 * Hoshi's semantics of a heading level 2, i.e, the `<h2>` component.
 *
 * @example
 * ```vue
 * <script setup lang="ts">
 * import { H2 } from '~/components/ui/typography';
 * </script>
 * <template>
 *     <H2>Hello, world!</H2>
 * </template>
 * ```
 */
export const H2 = createHeadingComponent(2);

/**
 * `<H3>{...content}</H3>`
 *
 * Hoshi's semantics of a heading level 3, i.e, the `<h3>` component.
 *
 * @example
 * ```vue
 * <script setup lang="ts">
 * import { H3 } from '~/components/ui/typography';
 * </script>
 * <template>
 *     <H3>Hello, world!</H3>
 * </template>
 * ```
 */
export const H3 = createHeadingComponent(3);

/**
 * `<H4>{...content}</H4>`
 *
 * Hoshi's semantics of a heading level 4, i.e, the `<h4>` component.
 *
 * @example
 * ```vue
 * <script setup lang="ts">
 * import { H4 } from '~/components/ui/typography';
 * </script>
 * <template>
 *     <H4>Hello, world!</H4>
 * </template>
 * ```
 */
export const H4 = createHeadingComponent(4);

/**
 * `<H5>{...content}</H5>`
 *
 * Hoshi's semantics of a heading level 5, i.e, the `<h5>` component.
 *
 * @example
 * ```vue
 * <script setup lang="ts">
 * import { H5 } from '~/components/ui/typography';
 * </script>
 * <template>
 *     <H5>Hello, world!</H5>
 * </template>
 * ```
 */
export const H5 = createHeadingComponent(5);

/**
 * `<H6>{...content}</H6>`
 *
 * Hoshi's semantics of a heading level 6, i.e, the `<h6>` component.
 *
 * @example
 * ```vue
 * <script setup lang="ts">
 * import { H6 } from '~/components/ui/typography';
 * </script>
 * <template>
 *     <H6>Hello, world!</H6>
 * </template>
 * ```
 */
export const H6 = createHeadingComponent(6);
