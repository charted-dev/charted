/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.tools.kotlin.rust

import kotlinx.datetime.LocalDateTime
import org.noelware.charted.databases.postgres.models.User
import org.noelware.charted.types.helm.RepoType
import kotlin.reflect.KClass
import kotlin.reflect.KProperty1
import kotlin.reflect.full.declaredMemberProperties
import kotlin.reflect.typeOf

/**
 * Represents the generator
 */
object Generator {
    inline fun <reified T : Any> generate(cls: KClass<T>): String {
        println("Generating Rust stub for class $cls!")

        if (cls.java.isEnum) {
            println("Detected that class $cls is a enum!")
            return buildString {
                appendLine("#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]")
                appendLine("pub enum ${cls.simpleName} {")
                for (c in cls.java.enumConstants) {
                    appendLine("   $c,")
                }

                appendLine("}")
            }
        }

        return buildString {
            appendLine("#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]")
            appendLine("#[serde(deny_unknown_fields)]")
            appendLine("pub struct ${cls.simpleName} {")
            for (member in cls.declaredMemberProperties) {
                val type = try {
                    member.asRustCounterpart()
                } catch (e: Exception) {
                    println("Skipping member ${member.name} due to ${e.message}")
                    continue
                }

                if (member.name == "type") {
                    appendLine("    #[serde(rename = \"${type.toSnakeCase()}\")]")
                    appendLine("    pub ${type.toSnakeCase()}: $type,")
                    continue
                }

                appendLine("    pub ${if (member.name == "ownerID") "owner_id" else member.name.toSnakeCase()}: $type,")
            }

            appendLine("}")
        }
    }
}

fun <T, V> KProperty1<T, V>.asRustCounterpart(): String = when (returnType) {
    typeOf<LocalDateTime?>() -> "Option<::chrono::DateTime<::chrono::Utc>>"
    typeOf<LocalDateTime>() -> "::chrono::DateTime<::chrono::Utc>"
    typeOf<RepoType>() -> "RepoType"
    typeOf<Boolean>() -> "bool"
    typeOf<String?>() -> "Option<String>"
    typeOf<String>() -> "String"
    typeOf<Long>() -> "u64"
    typeOf<User>() -> "User"
    typeOf<Int>() -> "u32"
    else -> error("Unable to determine Rust type from [$returnType]")
}

fun String.toSnakeCase(): String = "(?<=[a-zA-Z])[A-Z]".toRegex().replace(this) { "_${it.value}" }.lowercase()
