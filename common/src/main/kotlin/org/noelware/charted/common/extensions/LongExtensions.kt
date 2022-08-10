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
