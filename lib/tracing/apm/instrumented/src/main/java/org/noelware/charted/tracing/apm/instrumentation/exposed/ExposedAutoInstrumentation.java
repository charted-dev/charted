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

package org.noelware.charted.tracing.apm.instrumentation.exposed;

import static net.bytebuddy.matcher.ElementMatchers.named;
import static net.bytebuddy.matcher.ElementMatchers.takesArgument;
import static net.bytebuddy.matcher.ElementMatchers.takesArguments;

import co.elastic.apm.agent.sdk.ElasticApmInstrumentation;
import java.util.Collection;
import java.util.Collections;
import net.bytebuddy.asm.Advice;
import net.bytebuddy.description.method.MethodDescription;
import net.bytebuddy.description.type.TypeDescription;
import net.bytebuddy.matcher.ElementMatcher;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.exposed.sql.statements.Statement;

/**
 * Instrumentation plugin to trace over Exposed's transaction API.
 */
public class ExposedAutoInstrumentation extends ElasticApmInstrumentation {
    @Override
    public @NotNull ElementMatcher<? super TypeDescription> getTypeMatcher() {
        return named("org.jetbrains.exposed.sql.Transaction");
    }

    // https://github.com/JetBrains/Exposed/blob/2f6ce9c40b282a3b1f907ceaedd07e52991f849a/exposed-core/src/main/kotlin/org/jetbrains/exposed/sql/Transaction.kt#L136-L161
    @Override
    public @NotNull ElementMatcher<? super MethodDescription> getMethodMatcher() {
        return named("exec").and(takesArguments(2)).and(takesArgument(0, Statement.class));
    }

    @Override
    public @NotNull Collection<String> getInstrumentationGroupNames() {
        return Collections.singletonList("noelware-apm-plugins");
    }

    @Override
    public String getAdviceClassName() {
        return "org.noelware.charted.tracing.apm.instrumentation.exposed.ExposedAutoInstrumentation$AdviceClass";
    }

    public static class AdviceClass {
        @Advice.OnMethodEnter(suppress = Throwable.class, inline = false)
        public static Object onEnterHandle(@Advice.Argument(0) Statement<Object> stmt) {
            return null;
        }

        @Advice.OnMethodExit(suppress = Throwable.class, onThrowable = Throwable.class, inline = false)
        public static void onExitHandle(@Advice.Thrown Throwable thrown, @Advice.Enter Object scopeObject) {}
    }
}
