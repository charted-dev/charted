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

@file:DependsOn("it.krzeminski:github-actions-kotlin-dsl:0.38.0")

import it.krzeminski.githubactions.actions.actions.CheckoutV3
import it.krzeminski.githubactions.actions.actions.SetupJavaV3
import it.krzeminski.githubactions.actions.actions.SetupNodeV3
import it.krzeminski.githubactions.actions.gradle.GradleBuildActionV2
import it.krzeminski.githubactions.actions.gradle.WrapperValidationActionV1
import it.krzeminski.githubactions.domain.RunnerType
import it.krzeminski.githubactions.domain.triggers.PullRequest
import it.krzeminski.githubactions.domain.triggers.Push
import it.krzeminski.githubactions.domain.triggers.WorkflowDispatch
import it.krzeminski.githubactions.dsl.workflow
import it.krzeminski.githubactions.yaml.toYaml

val lintAndUnitWorkflow = workflow(
    "Linting and Unit Testing",
    listOf(
        WorkflowDispatch(),
        PullRequest(
            types = listOf(PullRequest.Type.Opened, PullRequest.Type.Synchronize),
            branches = listOf("issue/gh-**", "feat/**", "main"),
            pathsIgnore = listOf(
                ".github/**",
                ".vscode/**",
                "assets/**",
                ".gitignore",
                "**.md",
                "LICENSE",
                "renovate.json",
            ),
        ),
        Push(
            branches = listOf("issue/gh-**", "feat/**", "main"),
            pathsIgnore = listOf(
                ".github/**",
                ".vscode/**",
                "assets/**",
                ".gitignore",
                "**.md",
                "LICENSE",
                "renovate.json",
            ),
        ),
    ),
) {
    job(
        "spotless",
        "Spotless [\${{matrix.runner}}, Java \${{matrix.java-version}}]",
        runsOn = RunnerType.Custom("\${{matrix.runner}}"),
        strategyMatrix = mapOf(
            "java-version" to listOf("17", "19"),
            "runner" to listOf("windows-latest", "ubuntu-latest", "macos-latest", "self-hosted"),
        ),
    ) {
        uses("Checkout repository", CheckoutV3(submodules = true))
        uses(
            "Setup Java \${{matrix.java-version}}",
            SetupJavaV3(
                javaVersion = "\${{matrix.java-version}}",
                architecture = "\${{matrix.runner == 'self-hosted' && 'aarch64' || 'amd64'}}",
                distribution = SetupJavaV3.Distribution.Temurin,
            ),
        )

        // Node.js is required for Spotless to install and run Prettier
        uses(
            "Setup Node.js v19.x",
            SetupNodeV3(
                nodeVersion = "19.x",
                architecture = "\${{matrix.runner == 'self-hosted' && 'aarch64' || 'amd64'}}",
            ),
        )

        uses("Setup Gradle", GradleBuildActionV2())
        uses("Validate Gradle Wrapper", WrapperValidationActionV1())
        uses(
            "Lint code-base with Spotless",
            GradleBuildActionV2(
                arguments = "spotlessCheck --scan",
            ),
        )

        uses(
            "Compiles Kotlin source sets",
            GradleBuildActionV2(
                arguments = "compileKotlin --scan",
            ),
        )

        uses(
            "Compiles Java source sets",
            GradleBuildActionV2(
                arguments = "compileJava --scan",
            ),
        )

        uses(
            "Run unit and integration tests",
            GradleBuildActionV2(
                arguments = "test --scan",
            ),
        )
    }
}

println(lintAndUnitWorkflow.toYaml())
