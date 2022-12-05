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

import LogstashTransport from 'winston-logstash/lib/winston-logstash-latest';
import { createLogger, transports, format } from 'winston';
import { Configuration, read } from './config';
import { colors, rgb, styles } from 'leeks.js';
import type Transport from 'winston-transport';
import { Server } from './server';

const levelColor: Record<Configuration['logging']['level'], (t: string) => string> = {
  debug: (t) => styles.bold(`\x1b[38;2;241;204;209m${t.toUpperCase()}\x1b[0m`),
  error: (t) => styles.bold(`\x1b[38;2;166;76;76m${t.toUpperCase()}\x1b[0m`),
  silly: (t) => styles.bold(`\x1b[38;2;0;33;71m${t.toUpperCase()}\x1b[0m`),
  warn: (t) => styles.bold(`\x1b[38;2;234;234;208m${t.toUpperCase()}\x1b[0m`),
  info: (t) => styles.bold(`\x1b[38;2;81;81;140m${t.toUpperCase()}\x1b[0m`)
};

const grayColor = (t: string) => `\x1b[90m${t}\x1b[0m`;

const dateFormat = Intl.DateTimeFormat(undefined, {
  dateStyle: 'medium',
  timeStyle: 'long'
});

async function main() {
  const config = await read(process.env.CHARTED_WEB_CONFIG_PATH);
  const logger = createLogger({
    transports: [
      new transports.Console({
        level: 'silly',
        format: format.combine(
          format.label({ label: 'charted-web' }),
          format.timestamp(),
          format.errors(),
          format.printf(
            ({ level, message, timestamp, label }) =>
              `${grayColor(`[${dateFormat.format(new Date(timestamp))}]`)} ${grayColor('[')}${levelColor[level](
                level.padEnd(5, ' ')
              )}${grayColor(']')} ${grayColor('<')}${grayColor(label.padEnd(10, ' '))}${grayColor('>')} :: ${message}`
          )
        )
      }),
      config.logging.logstash !== undefined
        ? new LogstashTransport({
            node_name: config.logging.logstash!.node_name,
            meta: Object.fromEntries(config.logging.logstash!.metadata),
            max_connect_retries: config.logging.logstash!.max_connections,
            timeout_connect_retries: config.logging.logstash!.timeout_connect_retry_ms,
            retries: 10,
            ...(config.logging.logstash?.ssl !== undefined
              ? {
                  ssl_enable: true,
                  ca: config.logging.logstash!.ssl!.ca,
                  ssl_cert: config.logging.logstash!.ssl!.cert,
                  ssl_key: config.logging.logstash!.ssl!.key,
                  rejectUnauthorized: config.logging.logstash!.ssl!.reject_unauthorized,
                  ssl_passphrase: config.logging.logstash!.ssl!.passphrase
                }
              : {})
          })
        : (undefined as unknown as Transport)
    ].filter(Boolean)
  });

  const server = new Server(config, logger);
}

main();
