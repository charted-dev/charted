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

package org.noelware.charted.common;

import org.jetbrains.annotations.Nullable;

/**
 * Represents the host architecture.
 */
public enum Architecture {
    X64,
    AARCH64;

    /**
     * Returns the current host architecture.
     */
    @Nullable
    public static Architecture current() {
        final String arch = System.getProperty("os.arch");
        return switch (arch) {
            case "x86_64", "amd64" -> X64;
            case "aarch64", "arm64" -> AARCH64;
            default -> null;
        };
    }

    /**
     * Returns true if the host architecture is x86_64.
     */
    public boolean isX64() {
        return this == X64;
    }

    /**
     * Returns true if the host architecture is aarch64.
     */
    public boolean isAarch64() {
        return this == AARCH64;
    }

    @Override
    public String toString() {
        return "Architecture(%s)".formatted(isAarch64() ? "arm64" : "amd64");
    }
}
