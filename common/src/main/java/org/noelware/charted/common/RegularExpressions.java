/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

package org.noelware.charted.common;

import java.util.regex.Pattern;
import org.noelware.charted.common.regexp.RegularExpression;

/**
 * Common regular expressions that are used in charted-server to not repeat any.
 */
public class RegularExpressions {
    private RegularExpressions() {}

    /**
     * Creates and returns a new {@link RegularExpression} object of the repository name and username
     * regular expression given by the <code>raw</code> value.
     *
     * @param raw The raw value to test against
     * @return {@link RegularExpression} object
     */
    public static RegularExpression getNameRegex(String raw) {
        return getNameRegex(raw, true);
    }

    /**
     * Creates and returns a new {@link RegularExpression} object of the repository name and username
     * regular expression given by the <code>raw</code> value.
     *
     * @param raw The raw value to test against
     * @param includeNumbers If the regular expression should match with numbers also
     * @return {@link RegularExpression} object
     */
    public static RegularExpression getNameRegex(String raw, boolean includeNumbers) {
        return getNameRegex(raw, includeNumbers, 32);
    }

    /**
     * Creates and returns a new {@link RegularExpression} object of the repository name and username
     * regular expression given by the <code>raw</code> value.
     *
     * @param raw The raw value to test against
     * @param includeNumbers If the regular expression should match with numbers also
     * @param length The maximum length to go over
     * @return {@link RegularExpression} object
     */
    public static RegularExpression getNameRegex(String raw, boolean includeNumbers, int length) {
        return new RegularExpression(createUsernameRegex(includeNumbers, length), raw);
    }

    public static RegularExpression getPasswordRegex(String raw) {
        return new RegularExpression(Pattern.compile("^.*(?=.{8,})(?=.*[a-zA-Z])(?=.*\\d)?(?=.*[!#$%&? \"])?.*$"), raw);
    }

    private static Pattern createUsernameRegex(boolean includeNumbers, int length) {
        String pattern;
        if (includeNumbers) {
            pattern = "^([A-z]|-|_|\\d{0,9}){0,%d}".formatted(length);
        } else {
            pattern = "^([A-z]|-|_){0,%d}".formatted(length);
        }

        return Pattern.compile(pattern);
    }
}
