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

import { bold, dim, gray, isColorSupported, red } from 'colorette';
import { hasOwnProperty, isObject } from '@noelware/utils';
import { getCallSites, isSerializedError, SerializedError } from '../serializers/error';
import { userInfo } from 'os';
import { basename } from 'path';
import { inspect } from 'util';
import { levels } from 'pino';

/** Represents mappings of { pino_level => function to colourize } */
export const LEVEL_TO_COLOUR_MAPPINGS: Record<string, (t: string) => string> = {
  info: (t) =>
    isColorSupported
      ? bold(`\x1b[38;2;81;81;140m${t.toUpperCase().padEnd(5, ' ')}\x1b[0m`)
      : t.toUpperCase().padEnd(5, ' '),

  warn: (t) =>
    isColorSupported
      ? bold(`\x1b[38;2;233;233;130m${t.toUpperCase().padEnd(5, ' ')}\x1b[0m`)
      : t.toUpperCase().padEnd(5, ' '),

  error: (t) =>
    isColorSupported
      ? bold(`\x1b[38;2;166;76;76m${t.toUpperCase().padEnd(5, ' ')}\x1b[0m`)
      : t.toUpperCase().padEnd(5, ' '),

  debug: (t) =>
    isColorSupported
      ? bold(`\x1b[38;2;165;204;165m${t.toUpperCase().padEnd(5, ' ')}\x1b[0m`)
      : t.toUpperCase().padEnd(5, ' '),

  trace: (t) =>
    isColorSupported
      ? bold(`\x1b[38;2;0;33;71m${t.toUpperCase().padEnd(5, ' ')}\x1b[0m`)
      : t.toUpperCase().padEnd(5, ' ')
};

/** Returns the length to pad the end the logger's name with `' '`. */
export const MAX_NAME_LENGTH = 20;

/** Returns the date formatter for formatting dates */
export const TIME_FORMATTER = new Intl.DateTimeFormat('en-GB', {
  dateStyle: 'short',
  timeStyle: 'long',
  timeZone: process.env.TZ ?? 'UTC'
});

/**
 * Prettify the error entry given to us
 * @param error The error to prettify
 */
export const prettifyError = (
  error: SerializedError,
  { colors, depth }: { colors: boolean; depth: number } = {
    colors: isColorSupported,
    depth: 2
  }
) => {
  console.log('hi');
  const content = [`${colors ? bold(red(error.type + ':')) : error.type + ':'} ${error.message}`];
  const fileMappingCache: Record<string, boolean> = {};
  const IDENT = '     ';

  // Ignore node internals, might make it grey coloured
  for (const stack of error.stack.filter((s) => !s.filename.startsWith('node:'))) {
    if (hasOwnProperty(fileMappingCache, stack.filename)) {
      content.push(
        `${IDENT}${colors ? dim('~') : '~'} ${
          colors
            ? bold(dim(`${stack.filename}:${stack.line}:${stack.column}`))
            : `${stack.filename}:${stack.line}:${stack.column}`
        }`
      );
    } else {
      fileMappingCache[stack.filename] = true;
      content.push(
        `   ${colors ? bold(dim('~> in')) : '~> in'} ${
          colors
            ? bold(dim(stack.filename === '<script>' ? '<script>' : basename(stack.filename)))
            : stack.filename === '<script>'
            ? '<script>'
            : basename(stack.filename)
        }`
      );

      content.push(
        `${IDENT}${colors ? dim('~') : '~'} ${
          colors
            ? bold(dim(`${stack.filename}:${stack.line}:${stack.column}`))
            : `${stack.filename}:${stack.line}:${stack.column}`
        }`
      );
    }
  }

  const printCauseExceptions = (error: SerializedError) => {
    let indent = '  ';

    for (const cause of error.causes) {
      content.push(`${indent}Caused by: ${colors ? bold(red(cause.type + ':')) : cause.type + ':'} ${cause.message}`);

      // Ignore node internals, might make it grey coloured
      for (const stack of cause.stack.filter((s) => !s.filename.startsWith('node:'))) {
        if (hasOwnProperty(fileMappingCache, stack.filename)) {
          content.push(
            `${indent + '  '}${colors ? dim('~') : '~'} ${
              colors
                ? bold(dim(`${stack.filename}:${stack.line}:${stack.column}`))
                : `${stack.filename}:${stack.line}:${stack.column}`
            }`
          );
        } else {
          fileMappingCache[stack.filename] = true;
          content.push(
            `${indent + '  '}${colors ? bold(dim('~> in')) : '~> in'} ${
              colors
                ? bold(dim(stack.filename === '<script>' ? '<script>' : basename(stack.filename)))
                : stack.filename === '<script>'
                ? '<script>'
                : basename(stack.filename)
            }`
          );

          content.push(
            `${indent + '  '}${colors ? dim('~') : '~'} ${
              colors
                ? bold(dim(`${stack.filename}:${stack.line}:${stack.column}`))
                : `${stack.filename}:${stack.line}:${stack.column}`
            }`
          );
        }
      }

      indent += '  ';
    }
  };

  printCauseExceptions(error);
  return content.join('\n');
};

/**
 * Represents the record that the {@link formatLog} function excepts to be.
 */
export interface LogRecord {
  [x: string]: unknown;

  hostname: string;
  message: string;
  error?: SerializedError;
  level: number;
  name: string;
  time: number;
  pid: number;
}

const paintColor = (colored: string, fallback: string) => (isColorSupported ? colored : fallback);

/**
 * Formats the given chunk into a prettified log entry
 * @param chunk The chunk to use
 * @example
 * ```ts
 * import { formatLog } from '~/logging/transports/PrettyFormatter.js';
 *
 * formatLog({
 *    '@timestamp': 1670569907162,
 *    message: 'Hello, world!',
 *    error: new Error('some error lol'),
 *    level: 30,
 *    name: 'charted-web:server'
 * });
 *
 * // [timestamp] [level] [name      ] <username@hostname - pid> :: message
 * // Error: some error lol
 * //   - barking in chat !!!!
 * ```
 */
export const formatLog = (chunk: string | LogRecord) => {
  if (!isObject(chunk)) {
    chunk = JSON.parse(chunk);
  }

  const { hostname, level, message, name, pid, time, ...rest } = chunk as LogRecord;
  const { username } = userInfo();
  const levelInfo = { name: levels.labels[level], paint: LEVEL_TO_COLOUR_MAPPINGS[levels.labels[level]] };
  const date = new Date(time);

  //if (hasOwnProperty(rest, 'req') && hasOwnProperty(rest, 'res')) {}

  let final = `${paintColor(gray('['), '[')}${paintColor(
    gray(TIME_FORMATTER.format(date)),
    TIME_FORMATTER.format(date)
  )}${paintColor(gray(']'), ']')} ${paintColor(gray('['), '[')}${levelInfo.paint(levelInfo.name)}${paintColor(
    gray(']'),
    ']'
  )} ${paintColor(gray('['), '[')}${paintColor(
    gray(name.padEnd(MAX_NAME_LENGTH, ' ')),
    name.padEnd(MAX_NAME_LENGTH, ' ')
  )}${paintColor(gray(']'), ']')} ${paintColor(gray('<'), '<')}${username}@${hostname} - ${pid}${paintColor(
    gray('>'),
    '>'
  )} :: ${message}`;

  if (hasOwnProperty(rest, 'req')) {
    delete rest.reqId;

    const { id, method, url, remoteAddress, remotePort } = rest.req as Record<string, any>;
    final += ` ${paintColor(gray('['), '[')}${method} ${url} <${id}@${remoteAddress}:${remotePort}${paintColor(
      gray(']'),
      ']'
    )}`;

    delete rest.req;
  }

  if (hasOwnProperty(rest, 'res')) {
    const { res, responseTime } = rest as Record<string, any>;
    final += ` ${paintColor(gray('['), '[')}${gray(`${res.statusCode} in ${responseTime.toFixed(2)}ms`)} ${gray('(')}${
      rest.reqId
    }${gray(')')}${paintColor(gray(']'), ']')}`;

    delete rest.responseTime;
    delete rest.reqId;
    delete rest.res;
  }

  final += '\n';

  if (hasOwnProperty(rest, 'error')) {
    final += `${prettifyError(rest.error!)}\n`;
  }

  delete rest.error;
  if (Object.keys(rest).length > 0) {
    final += inspect(rest, { colors: isColorSupported, depth: 2 });
    final += '\n';
  }

  return final;
};
