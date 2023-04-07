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

package org.noelware.charted.testing.yamlTestRunner.directives.internal.statusCode;

import io.ktor.http.HttpStatusCode;
import java.util.List;
import org.jetbrains.annotations.NotNull;
import org.noelware.charted.testing.yamlTestRunner.YamlTestContext;
import org.noelware.charted.testing.yamlTestRunner.annotations.Directive;
import org.noelware.charted.testing.yamlTestRunner.directives.DirectiveExecutionResult;
import org.noelware.charted.testing.yamlTestRunner.directives.YamlDirective;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

@Directive(name = "hasStatusCode", descriptor = StatusCodeDescriptor.class, argc = 1)
public class StatusCodeYamlDirective implements YamlDirective<StatusCodeDescriptor> {
    private final Logger LOG = LoggerFactory.getLogger(StatusCodeYamlDirective.class);

    @Override
    public StatusCodeDescriptor getDescriptorInstance(
            @NotNull YamlTestContext context, @NotNull List<Object> arguments) {
        final Object statusCode = arguments.get(0);
        try {
            final int parsed = Integer.parseInt(statusCode.toString());
            final StatusCodeDescriptor descriptor = new StatusCodeDescriptor(parsed);

            if (descriptor.statusCode().getDescription().equals("Unknown Status Code")) {
                throw new RuntimeException("Invalid status code: %s".formatted(statusCode));
            }

            return descriptor;
        } catch (NumberFormatException nfe) {
            throw new RuntimeException("Unable to parse integer value %s".formatted(statusCode), nfe);
        }
    }

    @Override
    public DirectiveExecutionResult compile(
            @NotNull StatusCodeDescriptor descriptor,
            @NotNull YamlTestContext context,
            @NotNull List<Object> arguments) {
        if (descriptor.statusCode().getDescription().contains("Unknown Status Code")) {
            return new DirectiveExecutionResult.Failed(new RuntimeException(
                    "Invalid status code: %d".formatted(descriptor.statusCode().getValue())));
        }

        final HttpStatusCode code = descriptor.statusCode();
        final int passedIn = (int) arguments.get(0);

        return passedIn == code.getValue()
                ? DirectiveExecutionResult.Success.INSTANCE
                : new DirectiveExecutionResult.Failed(new Exception("%d != %d".formatted(passedIn, code.getValue())));
    }
}
