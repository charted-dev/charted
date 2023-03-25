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

import com.sun.tools.attach.AgentInitializationException;
import com.sun.tools.attach.AgentLoadException;
import com.sun.tools.attach.AttachNotSupportedException;
import com.sun.tools.attach.VirtualMachine;
import java.io.IOException;
import java.lang.instrument.Instrumentation;
import java.lang.management.ManagementFactory;
import java.util.concurrent.atomic.AtomicBoolean;
import net.bytebuddy.agent.builder.AgentBuilder;
import net.bytebuddy.implementation.MethodDelegation;
import net.bytebuddy.implementation.SuperMethodCall;
import net.bytebuddy.matcher.ElementMatchers;
import org.jetbrains.annotations.NotNull;
import org.noelware.charted.modules.tracing.Traceable;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class TracingAgent {
    private static final AtomicBoolean hasInit = new AtomicBoolean(false);
    private static final Logger LOG = LoggerFactory.getLogger(TracingAgent.class);

    public static void agentmain(@NotNull String args, @NotNull Instrumentation inst) {
        // If we have already initialized, then we will not do this again
        if (!hasInit.compareAndSet(false, true)) return;

        LOG.info("Initializing agent...");
        new AgentBuilder.Default()
                .type(ElementMatchers.any())
                .transform((builder, type, _loader, _jModule, _domain) -> builder.method(
                                ElementMatchers.isAnnotatedWith(Traceable.class))
                        .intercept(MethodDelegation.to(Interceptor.class).andThen(SuperMethodCall.INSTANCE)))
                .installOn(inst);
    }

    /**
     * Side-loads this {@link TracingAgent} onto the current Java Virtual Machine. Since the
     * tracing configuration is done by the API server, we can't statically load this from
     * the <code>-javaagent</code> JVM argument.
     */
    public static void doSideLoad()
            throws IOException, AttachNotSupportedException, AgentLoadException, AgentInitializationException {
        LOG.info("Starting side-loading this agent to the current JVM...");

        final String vmName = ManagementFactory.getRuntimeMXBean().getName();
        LOG.debug("Running off VM '{}'", vmName);

        final var pidIdx = vmName.indexOf('@');
        if (pidIdx == -1) {
            LOG.warn("Unable to infer JVM pid from runtime, not doing anything");
            return;
        }

        final String actualPid = vmName.substring(0, pidIdx);
        final String classpath = System.getProperty("java.class.path");

        String javaAgentJar = null;
        for (String item : classpath.split(":")) {
            LOG.trace("item '{}' (found: {})", item, item.contains("charted-tracing"));
            if (item.contains("charted-tracing")) {
                LOG.debug("Found tracing JAR from classpath [{}]", item);
                javaAgentJar = item;
                break;
            }
        }

        if (javaAgentJar == null) {
            LOG.warn("Unable to infer charted-tracing-{VERSION}.jar file from classpath, not doing anything");
            return;
        }

        final VirtualMachine vm = VirtualMachine.attach(actualPid);
        vm.loadAgent(javaAgentJar, "");
        vm.detach();
    }
}
