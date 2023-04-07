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

package org.noelware.charted.testing.yamlTestRunner.directives;

import org.jetbrains.annotations.Nullable;

/**
 * Result of a {@link YamlDirective} execution result.
 */
public interface DirectiveExecutionResult {
    /**
     * @return if the execution result was successful or not
     */
    boolean wasSuccessful();

    /**
     * Represents a failed execution result
     */
    class Failed implements DirectiveExecutionResult {
        private final transient Throwable exception;

        public Failed() {
            this(null);
        }

        public Failed(@Nullable Throwable ex) {
            this.exception = ex;
        }

        @Override
        public boolean wasSuccessful() {
            return false;
        }

        /**
         * @return {@link Throwable} that was thrown during the execution
         */
        @Nullable
        public Throwable exception() {
            return exception;
        }
    }

    /**
     * Successful execution result
     */
    class Success implements DirectiveExecutionResult {
        public static final Success INSTANCE = new Success();

        private Success() {}

        @Override
        public boolean wasSuccessful() {
            return true;
        }
    }
}
