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

import java.util.List;
import org.jetbrains.annotations.NotNull;
import org.noelware.charted.testing.yamlTestRunner.Descriptor;
import org.noelware.charted.testing.yamlTestRunner.YamlTestContext;

/**
 * Represents a directive that can be compiled into an expression that can be validated
 * through JUnit5 assertion functions.
 *
 * <p>
 * Directives can be compiled into a {@link DirectiveExecutionResult} which determines if the
 * directive was a success or a failure.
 *
 * <h2>Example</h2>
 * <pre>{@code
 * @Directive(name = "isMissing")
 * public class IsMissingYamlDirective implements YamlDirective<MatchAssertionDescriptor> {
 *      @Override
 *      public DirectiveExecutionResult compile(
 *          @NotNull MatchAssertionDescriptor descriptor,
 *          @NotNull YamlTestContext context,
 *          @Nullable Map<String, Object> arguments
 *      ) {
 *          // implementation for "isMissing()"
 *          return DirectiveExecutionResult.success();
 *      }
 * }
 * }</pre>
 */
public interface YamlDirective<D extends Descriptor> {
    /**
     * Returns a {@link Descriptor} that matches this YAML directive. This is made to
     * validate it at runtime to make sure we are passing the right directive descriptor
     * correctly.
     *
     * @param context Test context
     * @param arguments Arguments that was passed in
     * @return {@link Descriptor} instance that was validated
     */
    D getDescriptorInstance(@NotNull YamlTestContext context, @NotNull List<Object> arguments);

    /**
     * Compiles a directive and returns the {@link DirectiveExecutionResult} of it.
     *
     * @param descriptor Registered descriptor for this {@link YamlDirective}.
     * @param context {@link YamlTestContext} if you need it
     * @param arguments List of arguments that was provided by the test runner
     * @return {@link DirectiveExecutionResult} of the compilation result
     */
    DirectiveExecutionResult compile(
            @NotNull D descriptor, @NotNull YamlTestContext context, @NotNull List<Object> arguments);
}
