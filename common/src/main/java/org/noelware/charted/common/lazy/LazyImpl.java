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
 * Represents an implementation class for {@link Lazy<T>}.
 * @param <T> Resolvent type for lazily providing the value
 */
public class LazyImpl<T> implements Lazy<T> {
    private static final Object UNINIT = new Object();
    private static volatile Object _value = UNINIT;

    private final Lazy<T> provider;
    private final Object MUTEX = new Object();

    public LazyImpl(Lazy<T> provider) {
        this.provider = provider;
    }

    @SuppressWarnings("unchecked")
    @Override
    public T get() {
        synchronized (MUTEX) {
            if (_value == UNINIT) {
                _value = provider.get();
            }
        }

        return (T) _value;
    }
}
