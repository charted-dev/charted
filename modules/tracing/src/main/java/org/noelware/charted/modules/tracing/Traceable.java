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

package org.noelware.charted.modules.tracing;

/**
 * Annotation to declare on any method to provide tracing via the charted's Java Agent for
 * tracing.
 *
 * @author Noel Towa (cutie@floofy.dev)
 * @since 23.03.23
 */
public @interface Traceable {
    /**
     * @return Operation of this traceable method, defaults to nothing
     */
    String operation() default "";

    /**
     * @return name of this transaction, defaults to 'method class#method-name', i.e:
     * method TraceAgent#doTracing
     */
    String name() default "";
}
