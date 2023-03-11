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
import it.krzeminski.githubactions.domain.RunnerType
import it.krzeminski.githubactions.domain.actions.Action
import it.krzeminski.githubactions.domain.triggers.Cron
import it.krzeminski.githubactions.domain.triggers.Push
import it.krzeminski.githubactions.domain.triggers.Schedule
import it.krzeminski.githubactions.domain.triggers.WorkflowDispatch
import it.krzeminski.githubactions.dsl.workflow
import it.krzeminski.githubactions.yaml.toYaml

class InitCodeQLAction(private val languages: String? = null): Action<Action.Outputs>(
    actionName = "codeql-action/init",
    actionOwner = "github",
    actionVersion = "v2",
) {
    override fun toYamlArguments(): LinkedHashMap<String, String> = linkedMapOf(
        *listOfNotNull(
            languages?.let { "languages" to it },
        ).toTypedArray(),
    )

    override fun buildOutputObject(stepId: String): Outputs = Outputs(stepId)
}

class AutobuildAction: Action<Action.Outputs>(
    actionName = "codeql-action/autobuild",
    actionOwner = "github",
    actionVersion = "v2",
) {
    override fun toYamlArguments(): LinkedHashMap<String, String> = linkedMapOf()
    override fun buildOutputObject(stepId: String): Outputs = Outputs(stepId)
}

class AnalyzeCodeQLAction: Action<Action.Outputs>(
    actionName = "codeql-action/analyze",
    actionOwner = "github",
    actionVersion = "v2",
) {
    override fun toYamlArguments(): LinkedHashMap<String, String> = linkedMapOf()
    override fun buildOutputObject(stepId: String): Outputs = Outputs(stepId)
}

class QodanaScanAction: Action<Action.Outputs>(
    actionName = "qodana-action",
    actionOwner = "JetBrains",
    actionVersion = "v2022.3.4",
) {
    override fun toYamlArguments(): LinkedHashMap<String, String> = linkedMapOf(
        "args" to "--baseline,qodana.sarif.json,--save-report",
    )

    override fun buildOutputObject(stepId: String): Outputs = Outputs(stepId)
}

class UploadSarifResultAction: Action<Action.Outputs>(
    actionName = "codeql-action/upload-sarif",
    actionOwner = "github",
    actionVersion = "v2",
) {
    override fun toYamlArguments(): LinkedHashMap<String, String> = linkedMapOf(
        "sarif_file" to "\${{runner.temp}}/qodana/results/qodana.sarif.json",
    )

    override fun buildOutputObject(stepId: String): Outputs = Outputs(stepId)
}

val codeQualityWorkflow = workflow(
    "Code Quality",
    on = listOf(
        WorkflowDispatch(),
        Schedule(listOf(Cron("0 0 * * *"))),
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
        "codeql",
        "CodeQL (Java \${{matrix.java-version}} ~ \${{matrix.language}})",
        runsOn = RunnerType.UbuntuLatest,

        // At the moment, we don't have CodeQL enabled since it doesn't support
        // Kotlin 1.8.10, only 1.8.0
        condition = "\${{false}}",
        strategyMatrix = mapOf(
            "java-version" to listOf("17", "19"),
            "language" to listOf("java", "kotlin"),
        ),

        _customArguments = mapOf(
            "permissions" to mapOf(
                "security-events" to "write",
                "contents" to "read",
                "actions" to "read",
            ),
        ),
    ) {
        uses("Checkout", CheckoutV3())
        uses(
            "Setup Java \${{matrix.java-version}}",
            SetupJavaV3(
                javaVersion = "\${{matrix.java-version}}",
                distribution = SetupJavaV3.Distribution.Temurin,
            ),
        )

        // Node.js is required for Spotless
        uses("Setup Node.js 19.x", SetupNodeV3(nodeVersion = "19.x"))

        // Initialize CodeQL
        uses("Init CodeQL", InitCodeQLAction(languages = "\${{matrix.language}}"))
        uses("Autobuild", AutobuildAction())
        uses("Analyze", AnalyzeCodeQLAction())
    }

    job("qodana", "Qodana", runsOn = RunnerType.UbuntuLatest) {
        uses("Checkout", CheckoutV3(fetchDepth = CheckoutV3.FetchDepth.Value(0)))
        uses("Perform Qodana scan", QodanaScanAction())
        uses("Upload Sarif Results to GitHub", UploadSarifResultAction())
    }
}

println(codeQualityWorkflow.toYaml())
