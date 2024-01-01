/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
 * Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
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

import * as generated from "./generated";

/**
 * Generated TypeScript declarations from [charted-server](https://github.com/charted-dev/charted)'s
 * [OpenAPI document](https://github.com/charted-dev/charted/blob/main/assets/openapi.json).
 */
declare namespace charted {
  /**
   * Type alias to represent all available routes
   */
  export type Route = keyof generated.paths;

  /**
   * Version number of all the available API versions.
   */
  export type APIVersion = 1;

  /**
   * Prefix for all the {@link APIVersion api versions}.
   */
  export type APIVersionPrefix = `/v${APIVersion}`;

  /**
   * Represents a unit type that doesn't resolve anything.
   */
  export type Unit = never;

  /**
   * Represents an API error that might occur in a REST handler. It contains the {@link code}
   * and {@link message} elements to give a better understanding on what happened. You can read up
   * on all the codes here: https://charts.noelware.org/docs/server/current/api/reference#error-codes.
   */
  export interface ApiError<Details = Unit> {
    /** A code that can be looked up on why a request failed. You can view all available codes in [the documentation](https://charts.noelware.org/docs/server/latest/api#errors). */
    code: string;

    /** Detailed, humane message on why the request failed. */
    message: string;

    /** JSON-represented value for more context about the error. */
    details: Details;
  }

  /**
   * Union type of the response in a API request.
   *
   * * `success`: Whether if the request succeeded or not.
   * * `data`: If `success` is true, then some data might appear that you might want.
   * * `errors`: List of errors on why the request failed.
   */
  export type ApiResponse<
    Data = Unit,
    Errors extends ApiError<any>[] = ApiError[],
  > = { success: true; data: Data } | { success: false; errors: Errors };

  /**
   * A resource for personal-managed API tokens that is created by a User. This is useful
   * for command line tools or scripts that need to interact with charted-server, but
   * the main use-case is for the [Helm plugin](https://charts.noelware.org/docs/helm-plugin/current).
   */
  export type ApiKey = generated.components["schemas"]["ApiKey"];

  /**
   * Represents the skeleton of a `Chart.yaml` file.
   */
  export type Chart = generated.components["schemas"]["Chart"];

  /**
   * In Helm, one chart may depend on any number of other charts. These dependencies can be dynamically linked using the dependencies'
   * field in Chart.yaml or brought in to the charts/ directory and managed manually. The charts required by the current chart are defined
   * as a list in the dependencies field.
   */
  export type ChartDependency =
    generated.components["schemas"]["ChartDependency"];

  /**
   * Represents the specification for a Chart.yaml-schema from a `index.yaml` reference.
   */
  export type ChartIndexSpec =
    generated.components["schemas"]["ChartIndexSpec"];

  /**
   * Name and URL/email address combination as a maintainer. [ChartMaintainer::name] can be referenced
   * as a `NameOrSnowflake` union.
   */
  export type ChartMaintainer =
    generated.components["schemas"]["ChartMaintainer"];

  /**
   * The apiVersion field should be v2 for Helm charts that require at least Helm 3. Charts supporting previous Helm versions
   * have an apiVersion set to v1 and are still installable by Helm 3.
   */
  export type ChartSpecVersion =
    generated.components["schemas"]["ChartSpecVersion"];

  /**
   * Represents what type this chart is. Do keep in mind that `operator` is not supported by Helm, but is specific to the API server.
   * For serializing to valid Helm objects, `application` will be the replacement.
   */
  export type ChartType = generated.components["schemas"]["ChartType"];

  /**
   * Represents the distribution that this instance is running off from.
   */
  export type Distribution = generated.components["schemas"]["Distribution"];

  /**
   * Generic entrypoint message for any API routes like `/users`.
   */
  export type EntrypointResponse =
    generated.components["schemas"]["EntrypointResponse"];

  /**
   * ImportValues hold the mapping of source values to parent key to be imported. Each
   * item can be a child/parent sublist item or a string.
   */
  export type ImportValue = generated.components["schemas"]["ImportValue"];

  /**
   * Represents a resource that is correlated to a repository or organization member
   * that can control the repository's metadata.
   */
  export type Member = generated.components["schemas"]["Member"];

  /** Error kind for a {@link NameError name parsing error}. */
  export const NameErrorKind: {
    INVALID_CHARACTER: "invalid_character";
    EXCEEDED_MAX: "exceeded_max";
    INVALID_UTF8: "invalid_utf8";
    EMPTY: "empty";
  };

  /**
   * Represents a {@link Name} parsing error.
   */
  export class NameError extends Error {
    /**
     * @param kind error kind
     * @param message the message that gives the `kind` a humane meaning.
     * @param extra extra properties.
     */
    constructor(
      kind: (typeof NameErrorKind)[keyof typeof NameErrorKind],
      message: string,
      extra?: Record<string, unknown>,
    );

    /**
     * the error kind
     */
    public kind: (typeof NameErrorKind)[keyof typeof NameErrorKind];

    /**
     * @returns boolean to indiciate that this {@link NameError} might've exceeded the {@link Name} limit.
     */
    public isExceededMaximumAmount(): boolean;

    /**
     * @returns boolean to indicate that this {@link NameError} might be an invalid input.
     */
    public isInvalidInput(): boolean;

    /**
     * @returns boolean to indicate that this {@link NameError} might be a cause of an invalid utf-8 character.
     */
    public isInvalidUTF8(): boolean;

    /**
     * @returns boolean to indicate that this {@link NameError} might be from an empty string.
     */
    public isEmptyInput(): boolean;

    /**
     * @returns returns the extra characters that exceeded the amount, or `-1` if this was not
     * caused by a {@link NameErrorKind.EXCEEDED_MAX}.
     */
    public getExtraCharacters(): number;

    /**
     * @returns returns the input that was used. if this was caused by a {@link NameErrorKind.EMPTY},
     * then this will result in an empty string. if this was caused by a {@link NameErrorKind.INVALID_CHARACTER},
     * then it'll return the input; otherwise, an empty string is returned.
     */
    public getInput(): string;

    /**
     * @returns the index where the invalid character was at in the input, otherwise
     * `-1` is returned.
     */
    public getIndex(): number;

    /**
     * @returns the character where the invalid character was in the input if this was
     * caused by a {@link NameErrorKind.INVALID_CHARACTER}, otherwise `null` is returned.
     */
    public getCharacter(): string | null;
  }

  /**
   * A valid UTF-8 string that is used to identify a resource from the REST API in a humane fashion.
   * This is meant to help identify a resource without trying to calculate the resource's Snowflake on the first try.
   */
  export class Name {
    /**
     * A valid UTF-8 string that is used to identify a resource from the REST API in a humane fashion.
     * This is meant to help identify a resource without trying to calculate the resource's Snowflake on the first try.
     *
     * @param input The input of the name, or `undefined` to leave blank.
     * @example
     * ```ts
     * import { Name } from '@ncharts/types';
     *
     * const name = new Name();
     * name.validate(); // => NameError [empty]: input was empty.
     *
     * const weow = new Name('weow');
     * console.log(weow.toString()); // => 'weow'
     * ```
     */
    public constructor(input?: string);

    /**
     * Returns the actual input that was received.
     */
    get value(): string | undefined;
    set value(val: string);

    /**
     * Does the actual validation as constructors shouldn't throw errors
     * in my mind. This is used in the `toString` method and when inspecting
     * via Node.js' util.inspect.
     *
     * @throws {NameError} If the input was invalid, this will most likely be thrown.
     */
    validate(): void;
  }

  /** Represents a union enum that can hold a Snowflake and a Name, which is a String that is validated with the Name regex */
  export type NameOrSnowflake =
    generated.components["schemas"]["NameOrSnowflake"];

  /** The ordering to use when querying paginated REST calls. */
  export type OrderBy = generated.components["schemas"]["OrderBy"];

  /**
   * Represents a unified entity that can manage and own repositories outside
   * a User. Organizations to the server is used for business-related Helm charts
   * that aren't tied to a specific {@link User}.
   */
  export type Organization = generated.components["schemas"]["Organization"];

  /** Represents the information from a paginated REST call. */
  export type PageInfo = generated.components["schemas"]["PageInfo"];

  /** Represents a paginated REST call for the `GET /users/:id/organizations` */
  export type PaginatedOrganization =
    generated.components["schemas"]["PaginatedOrganization"];

  /** Represents a paginated REST call from the `GET /users/:idOrName/repositories` or `GET /organizations/:idOrName/repositories` endpoints. */
  export type PaginatedRepository =
    generated.components["schemas"]["PaginatedRepository"];
  export type Repository = generated.components["schemas"]["Repository"];

  /**
   * Represents a resource that contains a release from a {@link Repository} release. Releases
   * are a way to group releases of new versions of Helm charts that can be easily
   * fetched from the API server.
   *
   * Any repository can have an unlimited amount of releases, but tags cannot clash
   * into each other, so the API server will not accept it. Each tag should be
   * a SemVer 2 comformant string, parsing is related to how Cargo evaluates SemVer 2 tags.
   */
  export type RepositoryRelease =
    generated.components["schemas"]["RepositoryRelease"];
  export type Session = generated.components["schemas"]["Session"];

  /** Unique identifier for a resource. Based off the [Twitter Snowflake](https://blog.twitter.com/engineering/en_us/a/2010/announcing-snowflake) spec. */
  export type Snowflake = bigint;

  /**
   * Union enum that can contain a String or a {@link ImportValue} as the import source
   * for referencing parent key items to be imported.
   */
  export type StringOrImportValue =
    generated.components["schemas"]["StringOrImportValue"];

  /** Represents an account that can own {@link Repository repositories} and {@link Organization organizations}. */
  export type User = generated.components["schemas"]["User"];

  /**
   * Namespace for all response-related types.
   */
  export namespace responses {}
}

export = charted;
export as namespace charted;
