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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import gradle.kotlin.dsl.accessors._778d9fc59a6b8fe0d4bfb234a622a0a1.testImplementation
import gradle.kotlin.dsl.accessors._778d9fc59a6b8fe0d4bfb234a622a0a1.testRuntimeOnly
import groovy.lang.Closure
import org.gradle.api.tasks.testing.logging.TestExceptionFormat
import org.gradle.api.tasks.testing.logging.TestLogEvent

plugins {
    kotlin("jvm")
}

repositories {
    mavenCentral()
    mavenLocal()
}

dependencies {
    testImplementation("org.junit.jupiter:junit-jupiter-api:5.8.2")
    testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine:5.8.2")
    testImplementation("org.testcontainers:testcontainers:1.17.2")
    testImplementation("ch.qos.logback:logback-classic:1.2.11")
    testImplementation("ch.qos.logback:logback-core:1.2.11")
    testImplementation("org.slf4j:slf4j-api:1.7.36")
    testImplementation(kotlin("test"))
}

tasks.withType<Test>().configureEach {
    outputs.upToDateWhen { false }
    testLogging {
        events.addAll(listOf(TestLogEvent.PASSED, TestLogEvent.FAILED, TestLogEvent.SKIPPED))
        showStandardStreams = true
        exceptionFormat = TestExceptionFormat.FULL
    }
}
