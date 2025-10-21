package org.stardustmodding.rs4j.util

object NativeTools {
    external fun getString(ptr: Long): String?
    external fun getBool(ptr: Long): Boolean
    external fun getInt(ptr: Long): Int
    external fun getByte(ptr: Long): Byte
    external fun getLong(ptr: Long): Long
    external fun getDouble(ptr: Long): Double
    external fun getFloat(ptr: Long): Float
    external fun getChar(ptr: Long): Char
    external fun getShort(ptr: Long): Short

    external fun getObjectJni(ptr: Long, className: String): Any?

    inline fun <reified T> getObject(ptr: Long): T? {
        return getObjectJni(ptr, T::class.java.canonicalName) as T?
    }
}
