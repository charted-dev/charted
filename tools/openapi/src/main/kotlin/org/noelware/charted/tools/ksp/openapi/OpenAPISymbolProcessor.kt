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

package org.noelware.charted.tools.ksp.openapi

import com.google.devtools.ksp.isAbstract
import com.google.devtools.ksp.processing.Resolver
import com.google.devtools.ksp.processing.SymbolProcessor
import com.google.devtools.ksp.processing.SymbolProcessorEnvironment
import com.google.devtools.ksp.symbol.*
import com.google.devtools.ksp.validate
import org.noelware.charted.tools.ksp.openapi.annotations.ResponseDataType

/**
 * Represents the symbol processor for processing OpenAPI-related classes.
 */
class OpenAPISymbolProcessor(private val environment: SymbolProcessorEnvironment): SymbolProcessor {
    override fun process(resolver: Resolver): List<KSAnnotated> {
        environment.logger.info("Running the OpenAPI symbol processor...")
        if (resolver.getNewFiles().none()) {
            environment.logger.warn("Processor has been ran more than once.")
            return emptyList()
        }

        return process0(resolver)
    }

    private fun process0(resolver: Resolver): List<KSAnnotated> {
        // Get all schemas
        val schemas = resolver.getSymbolsWithAnnotation(ResponseDataType::class.qualifiedName ?: error("Unable to determine name of @ResponseDataType"))
        val unableToProcess = schemas.filterNot { it.validate() }

        for (symbol in schemas.filter { it is KSClassDeclaration && it.validate() }) {
            symbol.accept(SchemaEventVisitor(), Unit)
        }

        return unableToProcess.toList()
    }

    private inner class SchemaEventVisitor: KSVisitorVoid() {
        override fun visitClassDeclaration(classDeclaration: KSClassDeclaration, data: Unit) {
            if (classDeclaration.isAbstract()) {
                environment.logger.error("Class $classDeclaration can't be abstract with @ResponseDataType annotation.", classDeclaration)
            }

            if (classDeclaration.classKind != ClassKind.CLASS) {
                environment.logger.error("Class $classDeclaration has to be a class.")
            }
        }
    }
}
