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

import { readFile, writeFile } from 'fs/promises';
import { hostname, userInfo } from 'os';
import { hasOwnProperty } from '@noelware/utils';
import { existsSync } from 'fs';
import yaml from 'js-yaml';
import z from 'zod';

export const DEFAULT_NODE_NAME = (() => {
  // Check if `WINTERFOX_DEDI_NODE` exists in system environment
  if (hasOwnProperty(process.env, 'WINTERFOX_DEDI_NODE')) return process.env.WINTERFOX_DEDI_NODE!;

  // Check if NODE_NAME exists (applicable in Helm Chart)
  if (hasOwnProperty(process.env, 'NODE_NAME')) return process.env.NODE_NAME!;

  // Fallback to "user@hostname"
  const user = userInfo();
  return `${user.username}@${hostname()}`;
})();

export type Configuration = z.infer<typeof configSchema>;

const logstashSchema = z.object({
  host: z.string(),
  port: z.number(),
  type: z.enum(['tcp', 'udp', 'http']).default('tcp'),
  ssl: z
    .object({
      key: z.string(),
      cert: z.string(),
      ca: z.string().optional(),
      passphrase: z.string().optional(),
      reject_unauthorized: z.boolean().default(true)
    })
    .optional()
});

const loggingSchema = z.object({
  level: z.enum(['info', 'error', 'warn', 'debug', 'trace']).default('info'),
  logstash: logstashSchema.optional(),
  json: z
    .boolean()
    .default(process.env.NODE_ENV === 'production')
    .nullish()
});

const genericProxySchema = z.object({ path: z.string(), serverPath: z.string() });
const apiServerSchema = z.object({
  healthcheck: z
    .object({
      interval: z.union([z.number(), z.string()]).default('30s'),
      retry_attemps: z.number().default(1)
    })
    .nullish(),
  host: z.string().default('0.0.0.0'),
  port: z.number().min(1024).max(65536).default(3651),
  ssl: z
    .union([z.boolean(), z.object({})])
    .optional()
    .nullish()
});

const configSchema = z
  .object({
    sentry_dsn: z.string().url().nullish(),
    logging: loggingSchema.default({ level: 'info' }),
    charted: apiServerSchema.required(),
    proxy: z.array(genericProxySchema).optional(),
    server: z
      .object({
        host: z.string(),
        port: z.number()
      })
      .default({
        host: '0.0.0.0',
        port: 2134
      })
  })
  .strict();

export const read = async (path = './config.yml') => {
  if (!existsSync(path)) return create(path);

  const config: Configuration = yaml.load(await readFile(path, 'utf-8')) as any;
  await configSchema.parseAsync(config);

  return config;
};

export const create = async (path: string) => {
  const defaultConfig: Configuration = {
    logging: {
      level: 'info'
    },
    server: {
      host: '0.0.0.0',
      port: 2134
    },
    charted: {
      host: '0.0.0.0',
      port: 3651
    }
  };

  const dumped = yaml.dump(defaultConfig, { noArrayIndent: false, indent: 4 });
  await writeFile(
    path,
    `# This configuration file was pre-generated by the web UI server.
${dumped}`
  );

  return defaultConfig;
};
