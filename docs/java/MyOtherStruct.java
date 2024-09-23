package org.stardustmodding.rs4j.docs.example.complex;

public class MyOtherStruct implements ParentClass {
    private static native String jni_get_a(long ptr);
    private static native long jni_get_b(long ptr); // This is the pointer to MyOtherStruct
    private static native long jni_set_a(long ptr, String value);
    private static native long jni_set_b(long ptr, long value); // The pointer is `value`
    private native long jni_init_new();
    
    private long __ptr;

    public MyOtherStruct() {
        __ptr = jni_init_new();
    }

    private MyOtherStruct(long ptr) {
        __ptr = ptr;
    }

    public String getA() {
        return jni_get_a(__ptr);
    }

    public MyStruct getB() {
        return MyStruct.from(jni_get_b(__ptr), this, "b");
    }

    public void setA(String value) {
        __ptr = jni_set_a(__ptr, value);
    }

    public void setB(MyStruct value) {
        __ptr = jni_set_b(__ptr, value.getPointer());
    }

    @Override
    public void updateField(String field, long pointer) {
        if (field == "b") {
            __ptr = jni_set_b(__ptr, pointer);
        }
    }

    public long getPointer() {
        return __ptr;
    }

    public static MyOtherStruct from(long ptr) {
        return new MyOtherStruct(ptr);
    }
}
