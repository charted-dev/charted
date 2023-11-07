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

import type { ApiError } from '@ncharts/types';

export class ApiResponseError<D = never> extends Error {
    constructor(errors: ApiError<D>[]) {
        super();

        Error.captureStackTrace && Error.captureStackTrace(this, ApiResponseError);
        this.message = ApiResponseError._computeMessage(errors);
        this.name = 'ApiResponseError';
    }

    static _computeMessage<D>(errors: ApiError<D>[]) {
        let content = `Received ${errors.length} error${errors.length === 1 ? '' : 's'}\n`;
        for (let i = 0; i < errors.length; i++) {
            content += `~> #${i + 1}. errors[${errors[i].code}]: ${errors[i].message}`;
            if (errors[i].detail) {
                content += `\n    ~> detail: ${JSON.stringify(errors[i].detail)}`;
            }

            content += '\n';
        }

        return content;
    }
}
