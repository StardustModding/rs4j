# Redstone's Guide to JNI Fuckery (Now with pointers!)

This is the method I call "throwing around pointers until shit works".

I'm too lazy to write out the Rust JNI bindings so just imagine them.

Also, none of this is implemented yet, this is just a scheme for how I'm going to do it.

> # WARNING
> THIS IS NOT ACCURATE! The real, tested, and accurate code is in this directory!

## Simple Classes

### Rust

```rs
pub struct MyStruct {
    pub a: String,
    pub b: i32,
    pub c: f64,
}

impl MyStruct {
    pub fn new() -> Self {
        Self {
            a: String::new(),
            b: 0,
            c: 0.0,
        }
    }
}
```

### Bindings

```rs4j
class MyStruct {
    field a: String;
    field b: i32;
    field c: f64;

    static init fn new();
}
```

### Generated Java code

```java
public class MyStruct {
    private static native String jni_get_a(long ptr);
    private static native Integer jni_get_b(long ptr);
    private static native Double jni_get_c(long ptr);

    private static native long jni_set_a(long ptr, String value);
    private static native long jni_set_b(long ptr, Integer value);
    private static native long jni_set_c(long ptr, Double value);

    private native long jni_init_new();
    
    private long __ptr;

    public MyStruct() {
        __ptr = jni_init_new();
    }

    private MyStruct(long ptr) {
        __ptr = ptr;
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
    }

    public void setB(Integer value) {
        __ptr = jni_set_b(__ptr, value);
    }

    public void setC(Double value) {
        __ptr = jni_set_c(__ptr, value);
    }

    public static MyStruct from(long ptr) {
        return new MyStruct(ptr);
    }
}
```

---

## Complex Classes

### Rust

```rs
pub struct MyStruct {
    pub a: String,
    pub b: i32,
    pub c: f64,
}

pub struct MyOtherStruct {
    pub a: String,
    pub b: MyStruct,
}

impl MyStruct {
    pub fn new() -> Self {
        Self {
            a: String::new(),
            b: 0,
            c: 0.0,
        }
    }
}

impl MyOtherStruct {
    pub fn new() -> Self {
        Self {
            a: String::new(),
            b: MyStruct::new(),
        }
    }
}
```

### Bindings

```rs4j
class MyStruct {
    field a: String;
    field b: i32;
    field c: f64;

    static init fn new();
}

class MyOtherStruct {
    field a: String;
    field b: MyStruct;

    static init fn new();
}
```

### Generated Java code

```java
public interface ParentClass {
    void updateField(String field, long pointer);
}

public class MyStruct {
    private static native String jni_get_a(long ptr);
    private static native Integer jni_get_b(long ptr);
    private static native Double jni_get_c(long ptr);
    private static native long jni_set_a(long ptr, String value);
    private static native long jni_set_b(long ptr, Integer value);
    private static native long jni_set_c(long ptr, Double value);
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

    public static MyStruct from(long ptr) {
        return new MyStruct(ptr);
    }

    public static MyStruct from(long ptr, ParentClass parent, String parentField) {
        return new MyStruct(ptr, parent, parentField);
    }
}

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
```
