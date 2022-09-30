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

package org.noelware.charted.testing.kubernetes.junit;

import java.lang.annotation.*;
import org.junit.jupiter.api.extension.ExtendWith;

/**
 * Represents a JUnit annotation to instruct the Kubernetes extension to install the latest
 * version of Minikube, it might take a while depending on your internet connection. Use the {@link DisabledIfNoMinikube}
 * annotation to not run the tests at all.
 */
@Target(ElementType.TYPE)
@Retention(RetentionPolicy.RUNTIME)
@ExtendWith(NoelKubeExtension.class)
@Inherited
public @interface InstallMinikubeIfNotFound {}
