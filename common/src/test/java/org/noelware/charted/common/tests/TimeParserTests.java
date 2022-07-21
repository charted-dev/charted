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

package org.noelware.charted.common.tests;

import static org.junit.Assert.*;

import java.util.Optional;
import org.junit.Test;
import org.noelware.charted.common.TimeParser;

public class TimeParserTests {
    @Test
    public void test_ifParsesCorrectly() {
        var year = TimeParser.fromString("1 year");
        var week = TimeParser.fromString("1 week");
        var day = TimeParser.fromString("2 days");
        var minute = TimeParser.fromString("12 minutes");
        var second = TimeParser.fromString("22 seconds");
        var ms = TimeParser.fromString("25 milliseconds");

        assertEquals(Optional.of(31557600000L).get(), year);
        assertEquals(Optional.of(604800000L).get(), week);
        assertEquals(Optional.of(172800000L).get(), day);
        assertEquals(Optional.of(720000L).get(), minute);
        assertEquals(Optional.of(22000L).get(), second);
        assertEquals(Optional.of(25L).get(), ms);
    }

    @Test
    public void test_checkIfParsesCorrectlyByString() {
        var year = TimeParser.fromLong(31557600000L, true);
        var week = TimeParser.fromLong(604800000L, false);
        var day = TimeParser.fromLong(172800000L, true);
        var minute = TimeParser.fromLong(720000L, false);
        var second = TimeParser.fromLong(22000L, true);
        var ms = TimeParser.fromLong(22L, false);

        assertEquals(year, "1 year");
        assertEquals(week, "1w");
        assertEquals(day, "2 days");
        assertEquals(minute, "12min");
        assertEquals(second, "22 seconds");
        assertEquals(ms, "22ms");
    }
}
