/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
 * Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

import { z } from 'zod';

/**
 * Custom Zod refined validator for a entity [Name][name].
 *
 * [name]: https://charts.noelware.org/docs/server/latest/api/types#Name
 */
export const name = z.string().superRefine((input, ctx) => {
    if (input.length === 0) {
        return ctx.addIssue({
            code: z.ZodIssueCode.too_small,
            minimum: 1,
            type: 'string',
            inclusive: false,
            message: 'name received was empty'
        });
    }

    if (input.length > 32) {
        const exceeded = input.length - 32;
        return ctx.addIssue({
            code: z.ZodIssueCode.too_big,
            maximum: 32,
            inclusive: false,
            type: 'string',
            message: `name went over ${exceeded} characters, expected Name to contain 1..=32 in length`
        });
    }

    for (let i = 0; i < input.length; i++) {
        const ch = input[i];
        if (/^[a-z0-9]+$/i.test(ch)) {
            continue;
        }

        if (ch === '_') {
            continue;
        }

        if (ch === '-') {
            continue;
        }

        ctx.addIssue({
            code: z.ZodIssueCode.custom,
            message: `character '${ch}' in index ${i} was not a valid character; expected alphanumeric, '-', or '_'`
        });
    }
});
