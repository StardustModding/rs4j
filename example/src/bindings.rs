use rs4j::prelude::*;

#[allow(non_camel_case_types)]
pub struct __JNI_MyOtherStruct {
    pub a: String,
    pub b: *mut MyStruct,
}

impl __JNI_MyOtherStruct {
    pub unsafe fn to_rust(&self) -> MyOtherStruct {
        MyOtherStruct {
            a: self.a.clone(),
            b: (&mut *self.b).clone(),
        }
    }

    pub unsafe fn __wrapped_new() -> Self {
        let base = MyOtherStruct::new();

        Self {
            a: base.a.clone(),
            b: Box::leak(Box::new(base.b)) as *mut MyStruct,
        }
    }

    pub unsafe fn __wrapped_say_only(&self, message: String) -> () {
        MyOtherStruct::say_only(&self.to_rust(), message).clone()
    }

    pub unsafe fn __wrapped_say(&self, p2: String) -> () {
        MyOtherStruct::say(&self.to_rust(), p2).clone()
    }

    pub unsafe fn __wrapped_say_with(&self, p1: MyStruct, p2: String) -> () {
        MyOtherStruct::say_with(&self.to_rust(), p1, p2).clone()
    }
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
    missing_docs
)]
pub unsafe extern "system" fn Java_com_example_MyOtherStruct_jni_1set_1a<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
    val: JString<'local>,
) -> jlong {
    let it = &mut *(ptr as *mut __JNI_MyOtherStruct);
    let val = env.get_string(&val).unwrap().to_str().unwrap().to_string();

    it.a = val;

    ptr as jlong
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
    missing_docs
)]
pub unsafe extern "system" fn Java_com_example_MyOtherStruct_jni_1get_1a<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    let it = &*(ptr as *mut __JNI_MyOtherStruct);
    env.new_string(it.a.clone()).unwrap().as_raw()
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
    missing_docs
)]
pub unsafe extern "system" fn Java_com_example_MyOtherStruct_jni_1set_1b<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
    val: jlong,
) -> jlong {
    let it = &mut *(ptr as *mut __JNI_MyOtherStruct);

    it.b = val as *mut MyStruct;

    ptr as jlong
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
    missing_docs
)]
pub unsafe extern "system" fn Java_com_example_MyOtherStruct_jni_1get_1b<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
) -> jlong {
    let it = &mut *(ptr as *mut __JNI_MyOtherStruct);

    it.b as jlong
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
    missing_docs
)]
pub unsafe extern "system" fn Java_com_example_MyOtherStruct_jni_1init_1new<'local, >(mut env: JNIEnv<'local>, obj: JObject<'local>, ) -> jlong {
    
    let it = __JNI_MyOtherStruct::__wrapped_new();
    (Box::leak(Box::new(it)) as *mut __JNI_MyOtherStruct) as jlong
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
    missing_docs
)]
pub unsafe extern "system" fn Java_com_example_MyOtherStruct_jni_1say_1only<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, message: JString<'local>) -> () {
    let it = &*(ptr as *mut __JNI_MyOtherStruct);
    let message = env.get_string(&message).unwrap().to_str().unwrap().to_string();

    it.__wrapped_say_only(message.clone())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
    missing_docs
)]
pub unsafe extern "system" fn Java_com_example_MyOtherStruct_jni_1say<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, p2: JString<'local>) -> () {
    let it = &*(ptr as *mut __JNI_MyOtherStruct);
    let p2 = env.get_string(&p2).unwrap().to_str().unwrap().to_string();

    it.__wrapped_say(p2.clone())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
    missing_docs
)]
pub unsafe extern "system" fn Java_com_example_MyOtherStruct_jni_1say_1with<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, p1: jlong, p2: JString<'local>) -> () {
    let it = &*(ptr as *mut __JNI_MyOtherStruct);
    let p1 = &*(p1 as *mut MyStruct);
    let p2 = env.get_string(&p2).unwrap().to_str().unwrap().to_string();

    it.__wrapped_say_with(p1.clone(), p2.clone())
}

#[no_mangle]
#[allow(unused)]
pub unsafe extern "system" fn Java_com_example_MyOtherStruct_jni_1free<'local, >(_env: JNIEnv<'local>, _class: JClass<'local>, ptr: jlong) {
    let it = Box::from_raw(ptr as *mut __JNI_MyOtherStruct);
    let _ = Box::from_raw(it.b);
}

#[allow(non_camel_case_types)]
pub struct __JNI_MyStruct {
    pub a: String,
    pub b: i32,
    pub c: f64,
}

impl __JNI_MyStruct {
    pub unsafe fn to_rust(&self) -> MyStruct {
        MyStruct {
            a: self.a.clone(),
            b: self.b.clone(),
            c: self.c.clone(),
        }
    }

    pub unsafe fn __wrapped_new() -> Self {
        let base = MyStruct::new();

        Self {
            a: base.a.clone(),
            b: base.b.clone(),
            c: base.c.clone(),
        }
    }
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
    missing_docs
)]
pub unsafe extern "system" fn Java_com_example_MyStruct_jni_1set_1a<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
    val: JString<'local>,
) -> jlong {
    let it = &mut *(ptr as *mut __JNI_MyStruct);
    let val = env.get_string(&val).unwrap().to_str().unwrap().to_string();

    it.a = val;

    ptr as jlong
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
    missing_docs
)]
pub unsafe extern "system" fn Java_com_example_MyStruct_jni_1get_1a<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    let it = &*(ptr as *mut __JNI_MyStruct);
    env.new_string(it.a.clone()).unwrap().as_raw()
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
    missing_docs
)]
pub unsafe extern "system" fn Java_com_example_MyStruct_jni_1set_1b<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
    val: jint,
) -> jlong {
    let it = &mut *(ptr as *mut __JNI_MyStruct);

    it.b = val;

    ptr as jlong
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
    missing_docs
)]
pub unsafe extern "system" fn Java_com_example_MyStruct_jni_1get_1b<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
) -> jint {
    let it = &*(ptr as *mut __JNI_MyStruct);

    it.b as jint
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
    missing_docs
)]
pub unsafe extern "system" fn Java_com_example_MyStruct_jni_1set_1c<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
    val: jdouble,
) -> jlong {
    let it = &mut *(ptr as *mut __JNI_MyStruct);

    it.c = val;

    ptr as jlong
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
    missing_docs
)]
pub unsafe extern "system" fn Java_com_example_MyStruct_jni_1get_1c<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
) -> jdouble {
    let it = &*(ptr as *mut __JNI_MyStruct);

    it.c as jdouble
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
    missing_docs
)]
pub unsafe extern "system" fn Java_com_example_MyStruct_jni_1init_1new<'local, >(mut env: JNIEnv<'local>, obj: JObject<'local>, ) -> jlong {
    
    let it = __JNI_MyStruct::__wrapped_new();
    (Box::leak(Box::new(it)) as *mut __JNI_MyStruct) as jlong
}

#[no_mangle]
#[allow(unused)]
pub unsafe extern "system" fn Java_com_example_MyStruct_jni_1free<'local, >(_env: JNIEnv<'local>, _class: JClass<'local>, ptr: jlong) {
    let it = Box::from_raw(ptr as *mut __JNI_MyStruct);
    
}

