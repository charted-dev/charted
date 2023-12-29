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

import { hasOwnProperty } from '@noelware/utils';
import * as colors from 'colorette';
import {
    startGroup as __startGroup,
    endGroup as __endGroup,
    AnnotationProperties,
    warning as __warn,
    error as __error,
    info as __info
} from '@actions/core';

export const ci = hasOwnProperty(process.env, 'CI') && process.env.CI !== '';
export const startGroup: typeof __startGroup = (name) =>
    ci
        ? __startGroup(name)
        : (() => {
              console.info(
                  `${colors.isColorSupported ? colors.bold(colors.magenta('>>')) : '>>'} ${
                      colors.isColorSupported ? colors.cyan(name) : name
                  }`
              );
          })();

export const endGroup: typeof __endGroup = () => {
    if (ci) __endGroup();
};

export const info = (message: string) =>
    ci
        ? __info(message)
        : console.info(`${colors.isColorSupported ? colors.green('[info]') : '[info]'}       ${message}`);

export function warn(message: string): void;
export function warn(message: string, properties: AnnotationProperties): void;
export function warn(message: string, properties?: AnnotationProperties) {
    if (ci) {
        __warn(message, properties);
        return;
    }

    console.warn(`${colors.isColorSupported ? colors.yellow('[warning]') : '[warning]'}    ${message}`);
}

export function error(error: Error): void;
export function error(message: string): void;
export function error(message: string, properties: AnnotationProperties): void;
export function error(msgOrError: Error | string, properties?: AnnotationProperties) {
    if (ci) {
        __error(msgOrError, properties);
        return;
    }

    if (typeof msgOrError === 'string') {
        console.error(`${colors.isColorSupported ? colors.red('[error]') : '[error]'}      ${msgOrError}`);
        return;
    }

    if (msgOrError instanceof Error) {
        console.error(msgOrError);
        return;
    }

    console.error(`${colors.isColorSupported ? colors.red('[error]') : '[error]'}      ${JSON.stringify(msgOrError)}`);
}
