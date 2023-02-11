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

package org.noelware.charted.common.lazy;

/**
 * Represents a lazily evaluated expression that can only be initialized once.
 */
@FunctionalInterface
public interface Lazy<T> {
    /**
     * Lazily retrieves the value that was given in the lambda function.
     */
    T get();

    /**
     * Creates a new {@link Lazy<T>} value from the provider function given.
     * @param provider The provider function to retrieve the value.
     * @param <T> Resolvent type.
     */
    static <T> Lazy<T> create(Lazy<T> provider) {
        return new LazyImpl<>(provider);
    }
}
