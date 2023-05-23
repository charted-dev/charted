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

package org.noelware.charted.modules.tracing.agent;

import io.sentry.Sentry;
import java.lang.instrument.Instrumentation;
import java.util.List;
import java.util.Map;
import java.util.Set;
import net.bytebuddy.agent.builder.AgentBuilder;
import net.bytebuddy.agent.builder.ResettableClassFileTransformer;
import org.jetbrains.annotations.NotNull;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class InstallationListener implements AgentBuilder.InstallationListener {
    private final Logger LOG = LoggerFactory.getLogger(getClass());

    @Override
    public void onBeforeInstall(
            @NotNull Instrumentation instrumentation, @NotNull ResettableClassFileTransformer classFileTransformer) {}

    @Override
    public void onInstall(
            Instrumentation instrumentation, @NotNull ResettableClassFileTransformer classFileTransformer) {
        LOG.info("Successfully installed ByteBuddy instrumentation class [{}]", instrumentation.getClass());
    }

    @Override
    public Throwable onError(
            @NotNull Instrumentation instrumentation,
            @NotNull ResettableClassFileTransformer classFileTransformer,
            @NotNull Throwable throwable) {
        if (Sentry.isEnabled()) {
            Sentry.captureException(throwable);
        }

        LOG.error("Unable to setup instrumentation class [{}]:", instrumentation.getClass(), throwable);
        return null;
    }

    @Override
    public void onReset(
            @NotNull Instrumentation instrumentation, @NotNull ResettableClassFileTransformer classFileTransformer) {}

    @Override
    public void onBeforeWarmUp(
            @NotNull Set<Class<?>> types, @NotNull ResettableClassFileTransformer classFileTransformer) {
        LOG.trace(
                "before warm up on classes [{}]",
                String.join(", ", types.stream().map(Class::getName).toList()));
    }

    @Override
    public void onWarmUpError(
            @NotNull Class<?> type,
            @NotNull ResettableClassFileTransformer classFileTransformer,
            @NotNull Throwable throwable) {
        if (Sentry.isEnabled()) {
            Sentry.captureException(throwable);
        }

        LOG.error("Unable to warm up class [{}] for instrumentation:", type, throwable);
    }

    @Override
    public void onAfterWarmUp(
            @NotNull Map<Class<?>, byte[]> types,
            @NotNull ResettableClassFileTransformer classFileTransformer,
            boolean transformed) {
        final List<String> classes = types.keySet().stream().map(Class::getName).toList();
        LOG.trace("after warm up on classes [{}] [transformed: {}]", String.join(", ", classes), transformed);
    }
}
