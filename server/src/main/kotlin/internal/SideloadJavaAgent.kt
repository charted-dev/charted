package org.noelware.charted.server.internal

import dev.floofy.utils.slf4j.logging
import java.lang.management.ManagementFactory
import kotlin.system.exitProcess

val log by logging("org.noelware.charted.server.internal.SideLoadOpenTelJavaAgent")

fun sideLoadOtelJavaAgent() {
    log.info("Sideloading OpenTelemetry Java agent into this VM...")

    val vmName = ManagementFactory.getRuntimeMXBean().name
    log.debug("==> Running on VM [$vmName]")

    val pid = vmName.substring(0, vmName.indexOf('@'))
    log.debug(System.getProperty("java.class.path"))

    exitProcess(1)

//    try {
//        val vm = VirtualMachine.attach(pid)
//        vm.loadAgent("", "")
//    }
}

/*
        String nameOfRunningVM = ManagementFactory.getRuntimeMXBean().getName();
        int p = nameOfRunningVM.indexOf('@');
        String pid = nameOfRunningVM.substring(0, p);

        try {
            VirtualMachine vm = VirtualMachine.attach(pid);
            vm.loadAgent(jarFilePath, "");
            vm.detach();
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
 */
