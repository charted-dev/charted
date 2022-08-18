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

package org.noelware.charted.common.lazy;

public class LazyImpl<T> implements Lazy<T> {
    private static final Object UNINIT = new Object();
    private static volatile Object _instance = UNINIT;

    private final LazilyProvide<T> provider;
    private final Object _lock = new Object();

    public LazyImpl(LazilyProvide<T> provider) {
        this.provider = provider;
    }

    @SuppressWarnings("unchecked")
    @Override
    public T get() {
        synchronized (_lock) {
            if (_instance == UNINIT) {
                _instance = provider.get();
            }
        }

        return (T) _instance;
    }
}
