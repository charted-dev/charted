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

package org.noelware.charted.common;

import com.github.benmanes.caffeine.cache.Cache;
import com.github.benmanes.caffeine.cache.Caffeine;
import java.lang.reflect.Field;
import java.util.Objects;
import java.util.concurrent.TimeUnit;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * Utilities to perform reflection calls easily and efficiently.
 * @author Noel Towa (cutie@floofy.dev)
 * @since 04.04.23
 */
public class ReflectionUtils {
    private static final Logger LOG = LoggerFactory.getLogger(ReflectionUtils.class);

    // This might be even more inefficient since we shouldn't even do runtime reflection
    // most of the time, only if we need it but whatever.
    //
    // This just keeps a simple cache of reflection fields that expires after accessing
    // for ~10 minutes with a max of 20 fields allowed.
    private static final Cache<String, Object> _fieldCache = Caffeine.newBuilder()
            .maximumSize(100)
            .expireAfterAccess(10, TimeUnit.MINUTES)
            .build();

    // private instance, nu construct
    private ReflectionUtils() {}

    /**
     * Sets a file in a class instance.
     *
     * @param instance Instance to fetch the field from, and to set reflectively.
     * @param fieldName The name of the field, this must be accurate to the field you're
     *                  trying to look for, or it will log a warning if it couldn't
     *                  be found.
     *
     * @param value Actual value to set to the field.
     * @return Indication by a {@link Boolean boolean} if the field was set to what {@code value} was.
     * @param <C> Instance class type
     * @param <T> Field value type
     */
    public static <C, T> boolean setField(@NotNull C instance, @NotNull String fieldName, T value) {
        Objects.requireNonNull(instance);
        Objects.requireNonNull(fieldName);

        final Class<?> klazz = instance.getClass();
        LOG.trace("Finding field [{}] in class [{}]!", fieldName, klazz.getSimpleName());

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
            LOG.warn("Unable to set field [{}] in class [{}]", fieldName, klazz.getSimpleName(), e);
            return false;
        }
    }

    /**
     * Grabs and uses a field from any class instance, even if it is private
     * or not. This is useful mainly for tests for the configuration settings.
     *
     * @param instance Instance to fetch the field from.
     * @param inferTo Class reference to infer the field as, this will return null
     *                if the inferred class is not the same as the field's class.
     *
     * @param fieldName Accurate name of the field to fetch.
     * @return The actual field's instance, or {@code null} for any reason it couldn't be fetched.
     * @param <C> Instance class type
     * @param <T> Field value type
     * @throws ClassCastException If the inferred class is not the same as the one
     * that the field was using.
     */
    @Nullable
    public static <C, T> T getAndUse(@NotNull C instance, @NotNull Class<T> inferTo, @NotNull String fieldName) {
        Objects.requireNonNull(instance);
        Objects.requireNonNull(inferTo);
        Objects.requireNonNull(fieldName);

        final Class<?> klazz = instance.getClass();
        final String cacheKey = "%s#%s".formatted(klazz.getSimpleName(), fieldName);
        final Object cachedResult = _fieldCache.get(cacheKey, (key) -> {
            LOG.info(
                    "Finding field [{}] in class [{}], with infer class [{}]",
                    fieldName,
                    klazz.getSimpleName(),
                    inferTo.getCanonicalName());
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
            return inferTo.cast(cachedResult);
        }
    }
}
