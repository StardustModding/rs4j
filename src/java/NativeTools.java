package org.stardustmodding.rs4j.util;

import java.lang.reflect.ParameterizedType;

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

    private static native Object getObjectJni(long ptr, String className);

    @SuppressWarnings("unchecked")
    public static <T> T getObject(long ptr) {
        return (T) getObjectJni(ptr, new GetTypeParent<T>().getGenericName());
    }

    private static class GetTypeParent<T> {
        // FIXME: Does this even work? Isn't there type erasure during compilation?
        // Investigate.
        @SuppressWarnings("unchecked")
        public String getGenericName() {
            return ((Class<T>) ((ParameterizedType) getClass()
                    .getGenericSuperclass()).getActualTypeArguments()[0]).getName().replaceAll(".", "/");
        }
    }
}
