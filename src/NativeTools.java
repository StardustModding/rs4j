package org.stardustmodding.rs4j.util;

public class NativeTools {
    public static native String getString(long ptr);
    public static native boolean getBool(long ptr);
    public static native int getInt(long ptr);
    public static native byte getByte(long ptr);
    public static native long getLong(long ptr);
    public static native double getDouble(long ptr);
    public static native float getFloat(long ptr);
    public static native char getChar(long ptr);
    public static native short getShort(long ptr);
}
