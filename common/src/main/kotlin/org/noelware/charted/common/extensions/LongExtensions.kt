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

package org.noelware.charted.common.extensions

fun Long.formatToSize(long: Boolean = false): String {
    val kilo = this / 1024L
    val mega = kilo / 1024L
    val giga = mega / 1024L
    val tera = giga / 1024L

    return when {
        kilo < 1024 -> "${kilo.toDouble()}${if (long) " kilobytes" else "KiB"}"
        mega < 1024 -> "${mega.toDouble()}${if (long) " megabytes" else "MiB"}"
        giga < 1024 -> "${giga.toDouble()}${if (long) " gigabytes" else "GiB"}"
        tera < 1024 -> "${tera.toDouble()}${if (long) " terabytes" else "TiB"}"
        else -> "${toDouble()}${if (long) " bytes" else "B"}"
    }
}
