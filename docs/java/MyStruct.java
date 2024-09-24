package org.stardustmodding.rs4j.docs.example.complex;

public class MyStruct {
    private static native String jni_get_a(long ptr);
    private static native Integer jni_get_b(long ptr);
    private static native Double jni_get_c(long ptr);
    private static native long jni_set_a(long ptr, String value);
    private static native long jni_set_b(long ptr, Integer value);
    private static native long jni_set_c(long ptr, Double value);
    private static native void jni_free(long ptr);
    private native long jni_init_new();
    
    private long __ptr;
    private ParentClass __parent = null;
    private String __parentField = null;

    public MyStruct() {
        __ptr = jni_init_new();
    }

    private MyStruct(long ptr) {
        __ptr = ptr;
    }

    private MyStruct(long ptr, ParentClass parent, String parentField) {
        __ptr = ptr;
        __parent = parent;
        __parentField = parentField;
    }

    public String getA() {
        return jni_get_a(__ptr);
    }

    public Integer getB() {
        return jni_get_b(__ptr);
    }

    public Double getC() {
        return jni_get_c(__ptr);
    }

    public void setA(String value) {
        __ptr = jni_set_a(__ptr, value);

        if (__parent != null) {
            __parent.updateField(__parentField, __ptr);
        }
    }

    public void setB(Integer value) {
        __ptr = jni_set_b(__ptr, value);

        if (__parent != null) {
            __parent.updateField(__parentField, __ptr);
        }
    }

    public void setC(Double value) {
        __ptr = jni_set_c(__ptr, value);

        if (__parent != null) {
            __parent.updateField(__parentField, __ptr);
        }
    }

    public long getPointer() {
        return __ptr;
    }

    public void free() {
        jni_free(__ptr);
    }

    public static MyStruct from(long ptr) {
        return new MyStruct(ptr);
    }

    public static MyStruct from(long ptr, ParentClass parent, String parentField) {
        return new MyStruct(ptr, parent, parentField);
    }
}
