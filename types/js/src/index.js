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

const { TextDecoder, inspect } = require('util');
const assert = require('node:assert');
const utils = require('@noelware/utils');

/**
 * Represents the kind that a {@link NameError} can represent.
 */
const NameErrorKind = {
    INVALID_CHARACTER: 'invalid_character',
    EXCEEDED_MAX: 'exceeded_max',
    INVALID_UTF8: 'invalid_utf8',
    EMPTY: 'empty'
};

/**
 * Represents a {@link Name} parsing error.
 */
class NameError extends Error {
    #extra;

    /**
     * @param {string} kind error kind
     * @param {string} message the message that gives the `kind` a humane meaning.
     * @param {Record<string, unknown>} extra extra properties.
     */
    constructor(kind, message, extra = {}) {
        assert(Object.values(NameErrorKind).includes(kind), 'unknown NameError kind');
        super(`[${kind}]: ${message}`);

        Error.captureStackTrace(this, NameError);

        this.#extra = extra;
        this.kind = kind;
    }

    /**
     * @returns boolean to indicate that this {@link NameError} might be an invalid input.
     */
    isInvalidInput() {
        return this.kind === NameErrorKind.INVALID_CHARACTER;
    }

    /**
     * @returns boolean to indiciate that this {@link NameError} might've exceeded the {@link Name} limit.
     */
    isExceededMaximumAmount() {
        return this.kind === NameErrorKind.EXCEEDED_MAX;
    }

    /**
     * @returns boolean to indicate that this {@link NameError} might be a cause of an invalid utf-8 character.
     */
    isInvalidUTF8() {
        return this.kind === NameErrorKind.INVALID_UTF8;
    }

    /**
     * @returns boolean to indicate that this {@link NameError} might be from an empty string.
     */
    isEmptyInput() {
        return this.kind === NameErrorKind.EMPTY;
    }

    /**
     * @returns {number} returns the extra characters that exceeded the amount, or `-1` if this was not
     * caused by a {@link NameErrorKind.EXCEEDED_MAX}.
     */
    getExtraCharacters() {
        return this.isExceededMaximumAmount() && utils.hasOwnProperty(this.#extra, 'over') ? this.#extra.over : -1;
    }

    /**
     * @returns {string} returns the input that was used. if this was caused by a {@link NameErrorKind.EMPTY},
     * then this will result in an empty string. if this was caused by a {@link NameErrorKind.INVALID_CHARACTER},
     * then it'll return the input; otherwise, an empty string is returned.
     */
    getInput() {
        return this.isEmptyInput()
            ? ''
            : this.isInvalidInput() && utils.hasOwnProperty(this.#extra, 'input')
            ? this.#extra.input
            : '';
    }

    /**
     * @returns {number} the index where the invalid character was at in the input, otherwise
     * `-1` is returned.
     */
    getIndex() {
        return this.isInvalidInput() && utils.hasOwnProperty(this.#extra, 'at') ? this.#extra.at : -1;
    }

    /**
     * @returns {string | null} the character where the invalid character was in the input if this was
     * caused by a {@link NameErrorKind.INVALID_CHARACTER}, otherwise `null` is returned.
     */
    getCharacter() {
        return this.isInvalidInput() && utils.hasOwnProperty(this.#extra, 'ch') ? this.#extra.ch : null;
    }
}

/**
 * A valid UTF-8 string that is used to identify a resource from the REST API in a humane fashion.
 * This is meant to help identify a resource without trying to calculate the resource's Snowflake on the first try.
 *
 * ## Examples
 * ```js
 * import { Name } from '@ncharts/types';
 *
 * const name = new Name('weow');
 * name.validate(); // => doesn't throw a error
 *
 * const name2 = new Name('@@@@@');
 * name2.validate(); // => NameError [invalid_character]: input '@@@@@' is invalid at index 0, character '@'.
 *
 * const empty = new Name();
 * empty.validate(); // => NameError [empty]: input was empty.
 * ```
 */
class Name {
    #input;

    /**
     * @param {string | undefined} input the input.
     */
    constructor(input) {
        if (input !== undefined) {
            assert(typeof input === 'string', 'input received was not a string');
        }

        this.#input = input;
    }

    get value() {
        return this.#input;
    }

    set value(val) {
        assert(typeof val === 'string', 'value set was not a string');
        this.#input = val;
    }

    /**
     * Does the actual validation as constructors shouldn't throw errors
     * in my mind. This is used in the `toString` method and when inspecting
     * via Node.js' util.inspect
     */
    validate() {
        if (this.#input === undefined || this.#input.length === 0) {
            throw new NameError(NameErrorKind.EMPTY, 'input was empty');
        }

        // TODO(@auguwu): check if string is non utf-8
        // const decoder = new TextDecoder('utf-8', { fatal: true });
        // try {
        //     decoder.decode(Buffer.from(this.#input, 'utf-8'));
        // } catch (ex) {
        //     utils.assertIsError(ex);
        //     throw new NameError(NameErrorKind.INVALID_UTF8, 'received invalid utf-8 sequence', {
        //         input: this.#input,
        //         cause: ex
        //     });
        // }

        for (let i = 0; i < this.#input.length; i++) {
            if (/^[a-zA-Z0-9]$/i.test(this.#input[i])) {
                continue;
            }

            if (this.#input[i] === '-' || this.#input[i] === '_') {
                continue;
            }

            throw new NameError(NameErrorKind.INVALID_CHARACTER, 'received Name was invalid', {
                input: this.#input,
                ch: this.#input[i],
                at: i
            });
        }
    }

    [inspect.custom]() {
        this.validate();
        return `Name<${this.#input}>`;
    }

    toString() {
        this.validate();
        return this.#input;
    }
}

module.exports = { Name, NameError, NameErrorKind };
