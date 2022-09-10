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

package org.noelware.charted.gradle.plugins.golang;

import org.gradle.api.Action;
import org.gradle.process.ExecResult;
import org.gradle.process.ExecSpec;

/**
 * Represents an interface that has the #exec method from the {@link org.gradle.process.ExecOperations exec operations interface} or
 * a {@link org.gradle.api.Project Gradle project}.
 */
public interface ExecOperationsLike {
    ExecResult exec(Action<? super ExecSpec> action);
}
