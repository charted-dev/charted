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

package org.noelware.charted.modules.tracing.elastic;

import co.elastic.apm.attach.ElasticApmAttacher;
import java.lang.reflect.Field;
import java.net.InetAddress;
import java.net.UnknownHostException;
import java.util.*;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicReference;
import kotlinx.serialization.SerialName;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;
import org.noelware.charted.ChartedInfo;
import org.noelware.charted.configuration.kotlin.dsl.tracing.TracingConfig;
import org.noelware.charted.configuration.kotlin.dsl.tracing.apm.CircuitBreakerConfig;
import org.noelware.charted.configuration.kotlin.dsl.tracing.apm.Instrumentation;
import org.noelware.charted.modules.tracing.Tracer;
import org.noelware.charted.modules.tracing.Transaction;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * A {@link Tracer} module that its scope is only available to the Elastic APM Java agent.
 */
public class APMTracer implements Tracer {
    private final AtomicReference<Transaction> currentTransaction = new AtomicReference<>(null);
    private final AtomicBoolean hasStarted = new AtomicBoolean(false);
    private final Logger LOG = LoggerFactory.getLogger(getClass());
    private final TracingConfig.ElasticAPM settings;

    public APMTracer(TracingConfig.ElasticAPM settings) {
        this.settings = Objects.requireNonNull(settings, "APM settings shouldn't be null");
    }

    @Override
    public void init() {
        if (!hasStarted.compareAndSet(false, true)) {
            return;
        }

        LOG.info("Initialising APM agent...");

        final HashMap<String, String> apmConfig = new HashMap<>();
        final CircuitBreakerConfig circuitBreakerConfig = settings.getCircuitBreaker();
        if (circuitBreakerConfig != null && circuitBreakerConfig.getEnabled()) {
            insert(apmConfig, "circuit_breaker_enabled", String.valueOf(circuitBreakerConfig.getEnabled()));
            insert(
                    apmConfig,
                    "stress_monitor_system_cpu_relief_threshold",
                    String.valueOf(circuitBreakerConfig.getCpuReliefThreshold()));

            insert(
                    apmConfig,
                    "stress_monitor_system_cpu_stress_threshold",
                    String.valueOf(circuitBreakerConfig.getCpuStressThreshold()));

            insert(apmConfig, "stress_monitor_cpu_duration_threshold", circuitBreakerConfig.getCpuThreshold());
            insert(
                    apmConfig,
                    "stress_monitor_gc_relief_threshold",
                    String.valueOf(circuitBreakerConfig.getReliefThreshold()));

            insert(
                    apmConfig,
                    "stress_monitor_gc_stress_threshold",
                    String.valueOf(circuitBreakerConfig.getThreshold()));

            insert(apmConfig, "stress_monitoring_interval", circuitBreakerConfig.getInterval());
        }

        insert(apmConfig, "instrument", settings.getInstrument() ? "true" : "false");
        insert(apmConfig, "service_name", settings.getServiceName());
        insert(apmConfig, "application_packages", "org.noelware.charted");
        insert(apmConfig, "log_level", "INFO");

        final String nodeName = getServiceNodeName();
        if (!nodeName.isEmpty()) {
            insert(apmConfig, "service_node_name", nodeName);
        }

        final List<Instrumentation> enabledInstrumentation = settings.getEnabledInstrumentation();
        if (Instrumentation.EnumSet.INSTANCE.isWildcard(enabledInstrumentation)) {
            final List<String> instrumentation = Arrays.stream(Instrumentation.values())
                    .filter(inst -> inst != Instrumentation.Wildcard)
                    .map(APMTracer::getSerialNameForInst)
                    .filter(Objects::nonNull)
                    .toList();

            LOG.info("Configured instrumentation to be enabled (wildcard): [{}]", String.join(", ", instrumentation));
            insert(apmConfig, "enable_instrumentations", String.join(",", instrumentation));
        } else if (!enabledInstrumentation.isEmpty()) {
            final List<String> instrumentation = enabledInstrumentation.stream()
                    .map(APMTracer::getSerialNameForInst)
                    .filter(Objects::nonNull)
                    .toList();

            LOG.info("Configured instrumentation to be enabled: [{}]", String.join(", ", instrumentation));
            insert(apmConfig, "enable_instrumentations", String.join(",", instrumentation));
        }

        final List<Instrumentation> disabledInstrumentation = settings.getDisabledInstrumentation();
        if (Instrumentation.EnumSet.INSTANCE.isWildcard(disabledInstrumentation)) {
            final List<String> instrumentation = Arrays.stream(Instrumentation.values())
                    .filter(inst -> inst != Instrumentation.Wildcard)
                    .map(APMTracer::getSerialNameForInst)
                    .filter(Objects::nonNull)
                    .toList();

            LOG.info("Configured instrumentation to be disabled (wildcard): [{}]", String.join(", ", instrumentation));
            insert(apmConfig, "disable_instrumentations", String.join(",", instrumentation));
        } else if (!disabledInstrumentation.isEmpty()) {
            final List<String> instrumentation = disabledInstrumentation.stream()
                    .map(APMTracer::getSerialNameForInst)
                    .filter(Objects::nonNull)
                    .toList();

            LOG.info("Configured instrumentation to be disabled: [{}]", String.join(", ", instrumentation));
            insert(apmConfig, "disable_instrumentations", String.join(",", instrumentation));
        }

        final List<String> serverUrls = settings.getServerUrls();
        if (serverUrls.isEmpty()) {
            LOG.error("Missing 'config.tracing.apm.server_urls' configuration value, not enabling.");
            return;
        }

        if (serverUrls.size() == 1) {
            final String serverUrl = serverUrls.get(0);
            LOG.debug("Using a single APM server [{}]", serverUrl);

            insert(apmConfig, "server_url", serverUrl);
        } else {
            LOG.debug("Using multiple APM servers for fail-over [{}]", String.join(", ", serverUrls));
            insert(apmConfig, "server_urls", String.join(",", serverUrls));
        }

        LOG.trace("Using configuration options for APM agent: {}", apmConfig);
        ElasticApmAttacher.attach(apmConfig);
    }

    @NotNull
    @Override
    public AutoCloseable withTransaction(@NotNull String name, @Nullable String operation) {
        if (currentTransaction.get() != null) {
            throw new IllegalStateException("There is already an ongoing transaction, please use spans!");
        }

        final Transaction transaction = createTransaction(name, operation);
        currentTransaction.set(transaction);

        return () -> {
            transaction.end(null);
            currentTransaction.set(null);
        };
    }

    @NotNull
    @Override
    public AutoCloseable withTransaction(@NotNull String name) {
        return withTransaction(name, null);
    }

    @Nullable
    @Override
    public Transaction currentTransaction() {
        return currentTransaction.get();
    }

    @NotNull
    @Override
    public Transaction createTransaction(@NotNull String name) {
        return createTransaction(name, null);
    }

    @NotNull
    @Override
    public Transaction createTransaction(@NotNull String name, @Nullable String operation) {
        return new APMTransaction(name, operation, this);
    }

    @Override
    public void close() {}

    private void insert(HashMap<String, String> map, @NotNull String key, String value) {
        map.put(key, value);
    }

    private String getServiceNodeName() {
        // Check if we have `NODE_NAME`, we insert this in the Kubernetes
        // Helm chart (if applicable)
        final String k8sNodeName = System.getenv("NODE_NAME");
        if (k8sNodeName != null) {
            return k8sNodeName;
        }

        // Check if we have a dedicated node name (from the `WINTERFOX_NODE_NAME`
        // environment variable), Noelware uses this for their official instance,
        // so we will have it at higher priority.
        if (ChartedInfo.getDedicatedNode() != null) {
            return ChartedInfo.getDedicatedNode();
        }

        // Check if `config.tracing.apm.service_node_name` is set from
        // the configuration class.
        if (settings.getServiceNodeName() != null) {
            return settings.getServiceNodeName();
        }

        // Last resort: use the host name
        LOG.info("Getting service node name for APM via service hostname!");
        try {
            final InetAddress localhost = InetAddress.getLocalHost();
            final String hostName = localhost.getHostName();
            final String username = System.getProperty("user.name");

            if (hostName != null) {
                return "%s@%s".formatted(username, hostName);
            }

            LOG.warn("Unable to fetch service node name for APM, using empty string as last resort");
            return "";
        } catch (UnknownHostException e) {
            LOG.warn(
                    "Received UnknownHostException while trying to retrieve localhost, using empty string as last resort!",
                    e);

            return "";
        }
    }

    private static String getSerialNameForInst(Instrumentation instrumentation) {
        for (Field field : Instrumentation.class.getDeclaredFields()) {
            if (!field.isEnumConstant()) continue;
            if (field.getName().equals(instrumentation.name())) {
                final SerialName serialName = field.getAnnotation(SerialName.class);
                if (serialName == null) continue;

                return serialName.value();
            }
        }

        return null;
    }
}
