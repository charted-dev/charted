package org.noelware.charted.server.internal

import com.sun.tools.attach.VirtualMachine
import dev.floofy.utils.slf4j.logging
import java.lang.management.ManagementFactory

val log by logging("org.noelware.charted.server.internal.SideLoadOpenTelJavaAgent")

fun sideLoadOtelJavaAgent() {
    log.info("Sideloading OpenTelemetry Java agent into this VM...")

    val vmName = ManagementFactory.getRuntimeMXBean().name
    log.debug("==> Running on VM [$vmName]")

    val pid = vmName.substring(0, vmName.indexOf('@'))
    val classpath = System.getProperty("java.class.path")
    var javaAgentJar: String? = null

    for (item in classpath.split(':')) {
        if (item.contains("opentelemetry-javaagent")) {
            log.debug("Found OpenTelemetry agent in path [$item]")
            javaAgentJar = item
            break
        }
    }

    val vm = VirtualMachine.attach(pid)
    vm.loadAgent(javaAgentJar, "")
    vm.detach()
}
