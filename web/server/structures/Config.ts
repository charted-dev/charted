/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022 Noelware <team@noelware.org>
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

import * as yaml from 'js-yaml';
import z from 'zod';

const schema = z
  .object({
    sentry_dsn: z.string().url().nullable().default(null),
    server: z
      .object({
        headers: z.map(z.string(), z.string()),
        host: z.string(),
        port: z.number().max(65535).min(1024)
      })
      .strict()
      .default({
        headers: new Map(),
        host: '0.0.0.0',
        port: 2134
      }),

    charted: z
      .object({
        host: z.string(),
        port: z.number().max(65535).min(1024),
        ssl: z.boolean()
      })
      .strict()
      .default({
        host: '0.0.0.0',
        port: 3651,
        ssl: false
      })
  })
  .strict();

export type Config = z.infer<typeof schema>;
