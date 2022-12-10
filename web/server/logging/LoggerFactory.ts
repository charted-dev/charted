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

import pino, { Logger as PinoLogger, TransportTargetOptions } from 'pino';
import { errorSerializer, responseSerializer } from './serializers';
import { dirname, resolve } from 'path';
import { Configuration } from '~/config';
import { fileURLToPath } from 'url';
import { Logger } from './Logger';
import { hasOwnProperty } from '@noelware/utils';

/**
 * Represents a factory class for constructing {@link Logger loggers}. The root logger is made so any {@link get() #get()} invocations
 * will be a child logger of the root logger.
 *
 * @example
 * ```ts
 * import { useLoggerFactory } from '~/logging';
 *
 * const factory = useLoggerFactory();
 * const logger = factory.getLogger('main');
 * ```
 */
export class LoggerFactory {
  #cachedLoggers: Map<string, Logger> = new Map();
  #rootLogger: PinoLogger;

  constructor(config: Configuration) {
    const __dirname = dirname(fileURLToPath(import.meta.url));
    const targets: TransportTargetOptions[] = [];

    if ((hasOwnProperty(config.logging, 'json') && config.logging.json) || process.env.NODE_ENV === 'development') {
      targets.push({
        target: resolve(__dirname, `transports/console${process.env.NODE_ENV === 'production' ? '.js' : '.ts'}`),
        level: config.logging.level,
        options: {}
      });
    } else {
      targets.push({
        target: 'pino/file',
        level: config.logging.level,
        options: {}
      });
    }

    if (config.sentry_dsn !== undefined || config.sentry_dsn !== null) {
      targets.push({
        target: 'pino-sentry-transport',
        level: config.logging.level,
        options: {
          dsn: config.sentry_dsn
        }
      });
    }

    if (config.logging.logstash !== undefined) {
      if (['tcp', 'udp'].includes(config.logging.logstash!.type)) {
        targets.push({
          target: 'pino-socket',
          level: config.logging.level,
          options: {
            address: config.logging.logstash!.host,
            port: config.logging.logstash!.port,
            mode: config.logging.logstash!.type
          }
        });
      }
    }

    this.#rootLogger = pino(
      {
        name: 'main',
        level: config.logging.level,
        messageKey: 'message',
        serializers: {
          error: errorSerializer,
          res: responseSerializer,
          req: pino.stdSerializers.req
        }
      },
      pino.transport({ targets })
    );
  }

  get root() {
    return this.#rootLogger;
  }

  getLogger(...keys: string[]) {
    const name = keys.join(':');
    if (this.#cachedLoggers.has(name)) return this.#cachedLoggers.get(name)!;

    const logger = new Logger(name, this.root);
    this.#cachedLoggers.set(name, logger);

    return logger;
  }
}
