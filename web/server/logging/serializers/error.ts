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

import { hasOwnProperty, isObject } from '@noelware/utils';

// This is based off pino's standard serialization for errors, this is modified from
// the source code to include the "raw" error itself.

const SEEN = Symbol.for('$error:seen');

export const getCallSites = (error?: Error): NodeJS.CallSite[] => {
  const _prepareStackTrace = Error.prepareStackTrace;
  Error.prepareStackTrace = (_, stack) => stack;

  const stack = (error ?? new Error()).stack;
  Error.prepareStackTrace = _prepareStackTrace;

  // @ts-ignore
  return stack!.length === 1 ? stack : stack!.slice(1);
};

export interface SerializedError {
  aggregated?: SerializedError[];
  message: string;
  causes: SerializedError[];
  stack: SerializedCallSite[];
  type: string;
}

export interface SerializedCallSite {
  filename: string;
  function: string;
  line: number;
  column: number;
  isNative: boolean;
  isConstructor: boolean;
  isEvalScript: boolean;
}

export const isSerializedError = (error: unknown): error is SerializedError =>
  isObject(error) &&
  [
    typeof error['message'] === 'string',
    Array.isArray((error as SerializedError).stack),
    typeof error['type'] === 'string'
  ].some(Boolean) === false;

export const errorSerializer = (error: unknown) => {
  if (!(error instanceof Error)) return error;

  error[SEEN] = undefined;
  const err = Object.create(null);
  err.type =
    Object.prototype.toString.call(error.constructor) === '[object Function]'
      ? (error.constructor as Function).name // eslint-disable-line @typescript-eslint/ban-types
      : error.name;

  err.message = error.message;
  err.stack = getCallSites(error).map((callsite) => ({
    filename: callsite.getFileName() ?? '<script>',
    function: callsite.getFunctionName() ?? '(anonymous)',
    line: callsite.getLineNumber() ?? -1,
    column: callsite.getColumnNumber() ?? -1,
    isNative: callsite.isNative(),
    isConstructor: callsite.isConstructor(),
    isEvalScript: callsite.isEval()
  }));

  if (
    globalThis.AggregateError !== undefined &&
    error instanceof globalThis.AggregateError &&
    Array.isArray(error.errors)
  ) {
    err.aggregated = error.errors.map(errorSerializer);
  }

  if (error.cause !== undefined) {
    const DEPTH = 3;
    let i = 0;

    // eslint-disable-next-line no-constant-condition
    while (true) {
      if (i === DEPTH) break;

      if (!hasOwnProperty(err, 'causes')) err.causes = [];
      err.causes.push(errorSerializer(error.cause));
    }
  }

  delete err[SEEN];
  return err;
};
