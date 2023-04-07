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

package org.noelware.charted.testing.yamlTestRunner;

import java.util.List;
import java.util.Map;
import org.jetbrains.annotations.NotNull;
import org.noelware.charted.testing.yamlTestRunner.directives.YamlDirective;

/**
 * Context object for the {@link YamlTestRunner} to retrieve information at runtime, i.e,
 * all the directives that are available during execution.
 */
public interface YamlTestContext {
    /**
     * @return list of all the registered descriptors available
     */
    @NotNull
    List<YamlDirective<?>> directives();

    /**
     * Adds an inline variable to this test context
     * @param key The key to provide
     * @param value Value to insert, must have a valid {@link Object#toString()} implementation
     */
    void addInlineVariable(String key, Object value);

    /**
     * @return mapping of all the inline variables that are present
     */
    Map<String, Object> inlineVariables();

    /**
     * @return test context execution name
     */
    @NotNull
    String name();
}
