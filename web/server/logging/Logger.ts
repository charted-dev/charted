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

import { levels, Logger as PinoLogger } from 'pino';

/**
 * Represents a interface to define the base functions for the logger.
 */
interface BaseLogger {
  /**
   * Writes a `TRACE` level to the main logger
   * @param messages The messages to send
   */
  trace(...messages: unknown[]): void;

  /**
   * Writes a `DEBUG` level to the main logger
   * @param messages The messages to send
   */
  debug(...messages: unknown[]): void;

  /**
   * Writes a `ERROR` level to the main logger
   * @param messages The messages to send
   */
  error(...messages: unknown[]): void;

  /**
   * Writes a `WARN` level to the main logger
   * @param messages The messages to send
   */
  warn(...messages: unknown[]): void;

  /**
   * Writes a `INFO` level to the main logger
   * @param messages The messages to send
   */
  info(...messages: unknown[]): void;

  /**
   * Creates a new {@link BaseLogger child logger} with the given settings but with
   * a different logger name.
   *
   * @param keys The name to use.
   */
  child(...keys: string[]): Logger;

  /**
   * The name of this {@link BaseLogger logger}.
   */
  name: string;
}

export class Logger implements BaseLogger {
  ['constructor']!: typeof Logger;

  #inner: PinoLogger;
  constructor(public name: string, inner: PinoLogger) {
    this.#inner = inner.child({ name });
  }

  get inner() {
    return this.#inner;
  }

  trace(...messages: unknown[]) {
    return this.#inner.trace.apply(this.#inner, messages as unknown as any);
  }

  debug(...messages: unknown[]) {
    return this.#inner.debug.apply(this.#inner, messages as unknown as any);
  }

  error(...messages: unknown[]) {
    return this.#inner.error.apply(this.#inner, messages as unknown as any);
  }

  warn(...messages: unknown[]) {
    return this.#inner.warn.apply(this.#inner, messages as unknown as any);
  }

  info(...messages: unknown[]) {
    return this.#inner.info.apply(this.#inner, messages as unknown as any);
  }

  child(...keys: string[]) {
    return new this.constructor([this.name, ...keys].join(':'), this.#inner);
  }
}
