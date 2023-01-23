/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

package org.noelware.charted.common;

import com.github.benmanes.caffeine.cache.Cache;
import com.github.benmanes.caffeine.cache.Caffeine;
import java.lang.reflect.Field;
import java.util.concurrent.TimeUnit;
import org.jetbrains.annotations.Nullable;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * Utilities to access values through reflection.
 */
public class ReflectionUtils {
    private static final Logger LOG = LoggerFactory.getLogger(ReflectionUtils.class);

    // Keep a simple cache for reflection fields (to improve performance when performing reflection checks)
    // class#field_name => java.lang.reflect.Field
    //
    // since the ReflectionUtil shouldn't be accessed a whole ton, the max size should be
    // >=100 entries and cache entries expire in 2 hours.
    private static final Cache<String, Object> _fieldCache = Caffeine.newBuilder()
            .maximumSize(100)
            .expireAfterAccess(2, TimeUnit.HOURS)
            .build();

    // private instance, nu look
    private ReflectionUtils() {}

    /**
     * Reflectively set a value from the instance variable to whatever <code>value</code> is.
     * @param instance The instance variable to set the field on
     * @param fieldName Field name
     * @param value Field value
     * @return {@link Boolean boolean} to indicate the success if the value was set or not.
     */
    public static <C, T> boolean setField(C instance, String fieldName, T value) {
        final Class<?> klazz = instance.getClass();
        LOG.info("Finding field [{}] in class [{}]!", fieldName, klazz.getSimpleName());

        Field field;
        try {
            field = klazz.getDeclaredField(fieldName);
            field.setAccessible(true);
        } catch (NoSuchFieldException ignored) {
            LOG.warn("Field [{}] was not found in class [{}], not doing anything", fieldName, klazz.getSimpleName());
            return false;
        }

        try {
            field.set(instance, value);
            return true;
        } catch (IllegalAccessException e) {
            LOG.warn("Unable to set field [{}] in class [{}]: {}", fieldName, klazz.getSimpleName(), e);
            return false;
        }
    }

    /**
     * Fetch and use the field from the given {@link Class<C> class}.
     *
     * @param instance   The instance to get the field from
     * @param inferClass The class to infer the given result as.
     * @param fieldName  field name
     * @param <C>        <code>instance</code> type
     * @param <T>        <code>inferred</code> type
     * @return           The given field, or <code>null</code> if the field was not found, and it can't be cached.
     */
    @Nullable
    public static <C, T> T getAndUseField(C instance, Class<T> inferClass, String fieldName) {
        final Class<?> klazz = instance.getClass();
        final String cacheKey = "%s#%s".formatted(klazz.getSimpleName(), fieldName);
        final Object cachedResult = _fieldCache.get(cacheKey, (key) -> {
            LOG.info("Finding field [{}] in class [{}], with infer class [{}]", fieldName, klazz.getSimpleName(), inferClass);
            try {
                final Field field = klazz.getDeclaredField(fieldName);
                field.setAccessible(true);

                return field.get(instance);
            } catch (NoSuchFieldException e) {
                return null;
            } catch (IllegalAccessException e) {
                throw new RuntimeException(e);
            }
        });

        if (cachedResult == null) {
            LOG.debug("Unable to find field [{}] in class [{}]", fieldName, klazz.getSimpleName());
            return null;
        } else {
            LOG.debug("Found field [{}] in class [{}]!", fieldName, klazz.getSimpleName());
            return inferClass.cast(cachedResult);
        }
    }
}
