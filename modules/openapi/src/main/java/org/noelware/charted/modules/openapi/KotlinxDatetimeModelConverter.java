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

package org.noelware.charted.modules.openapi;

import com.fasterxml.jackson.databind.JavaType;
import com.fasterxml.jackson.databind.ObjectMapper;
import io.swagger.v3.core.converter.AnnotatedType;
import io.swagger.v3.core.converter.ModelConverter;
import io.swagger.v3.core.converter.ModelConverterContext;
import io.swagger.v3.core.jackson.ModelResolver;
import io.swagger.v3.oas.models.media.DateSchema;
import io.swagger.v3.oas.models.media.DateTimeSchema;
import io.swagger.v3.oas.models.media.Schema;
import java.util.Iterator;
import kotlinx.datetime.Instant;
import kotlinx.datetime.LocalDateTime;

public class KotlinxDatetimeModelConverter extends ModelResolver {
    public KotlinxDatetimeModelConverter(ObjectMapper mapper) {
        super(mapper);
    }

    @SuppressWarnings("rawtypes")
    @Override
    public Schema resolve(AnnotatedType type, ModelConverterContext context, Iterator<ModelConverter> chain) {
        if (type.isSchemaProperty()) {
            final JavaType typeResolved = objectMapper().constructType(type.getType());
            if (typeResolved != null) {
                final Class<?> finalized = typeResolved.getRawClass();
                if (LocalDateTime.class.isAssignableFrom(finalized)) {
                    final Schema schema = new DateTimeSchema();
                    super.resolveSchemaMembers(schema, type);

                    return schema;
                }

                if (Instant.class.isAssignableFrom(finalized)) {
                    final Schema schema = new DateSchema();
                    super.resolveSchemaMembers(schema, type);

                    return schema;
                }
            }
        }

        return chain.hasNext() ? chain.next().resolve(type, context, chain) : null;
    }
}
