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

package org.noelware.charted.testing.yamlTestRunner.annotations;

import org.noelware.charted.testing.yamlTestRunner.Descriptor;

/**
 * Annotation to apply metadata to a {@link org.noelware.charted.testing.yamlTestRunner.directives.YamlDirective}.
 */
public @interface Directive {
    /**
     * @return descriptor class for this directive, this must be a valid {@link Descriptor}
     * or it will fail at runtime.
     */
    Class<? extends Descriptor> descriptor();

    /**
     * @return the name of the directive, must be a valid camelCase function
     * name.
     */
    String name();

    /**
     * @return argument count when the directive was called, defaults to
     * 0.
     */
    int argc() default 0;
}
