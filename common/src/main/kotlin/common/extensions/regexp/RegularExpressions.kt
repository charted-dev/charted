/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.common.extensions.regexp

// #region Passwords
public val PASSWORD_REGEX: Regex = """^.*(?=.{8,})(?=.*[a-zA-Z])(?=.*\d)?(?=.*[!#$%&? "])?.*$""".toRegex()
// #endregion

// #region Names (Organization Name/Usernames)
public val NAME_SNOWFLAKE_REGEX: Regex = """^([A-z]|-|_|\d{0,9}){0,32}""".toRegex()
public val NAME_REGEX: Regex = """^([A-z]|-|_){0,32}""".toRegex()
// #endregion

// #region Repositories
public val REPOSITORY_SNOWFLAKE_REGEX: Regex = """^([A-z]|-|_|\d{0,9}){0,24}""".toRegex()
public val REPOSITORY_REGEX: Regex = """^([A-z]|-|_){0,24}""".toRegex()
// #endregion

public fun String.matchesNameRegex(): Boolean = this matches NAME_REGEX
public fun String.matchesNameAndIdRegex(): Boolean = this matches NAME_SNOWFLAKE_REGEX

public fun String.matchesRepoNameRegex(): Boolean = this matches REPOSITORY_REGEX
public fun String.matchesRepoNameAndIdRegex(): Boolean = this matches REPOSITORY_SNOWFLAKE_REGEX

public fun String.matchesPasswordRegex(): Boolean = this matches PASSWORD_REGEX
