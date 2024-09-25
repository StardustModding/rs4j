use jni::{
    objects::{JClass, JObject, JString},
    sys::{jlong, jstring},
    JNIEnv,
};

use crate::{MyOtherStruct, MyStruct};

// ========================= Wrapper =========================

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct __JNI_MyOtherStruct {
    pub a: String,
    pub b: *mut MyStruct,
}

impl __JNI_MyOtherStruct {
    pub fn new() -> Self {
        let it = MyOtherStruct::new();

        Self {
            a: it.a,
            b: Box::leak(Box::new(it.b)) as *mut MyStruct,
        }
    }

    pub unsafe fn to_rust(&self) -> MyOtherStruct {
        MyOtherStruct {
            a: self.a.clone(),
            b: (&mut *self.b).clone(),
        }
    }

    pub unsafe fn __wrapped_say_only(&self, message: String) {
        self.to_rust().say_only(message);
    }

    pub unsafe fn __wrapped_say(&self, p2: String) {
        self.to_rust().say(p2);
    }

    pub unsafe fn __wrapped_say_with(&self, p1: MyStruct, p2: String) {
        self.to_rust().say_with(p1, p2);
    }
}

// ========================= Getters =========================

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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_docs_example_complex_MyOtherStruct_jni_1get_1a<
    'local,
>(
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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_docs_example_complex_MyOtherStruct_jni_1get_1b<
    'local,
>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
) -> jlong {
    let it = &*(ptr as *mut __JNI_MyOtherStruct);
    it.b as jlong
}

// ========================= Setters =========================

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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_docs_example_complex_MyOtherStruct_jni_1set_1a<
    'local,
>(
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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_docs_example_complex_MyOtherStruct_jni_1set_1b<
    'local,
>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
    val: jlong,
) -> jlong {
    let it = &mut *(ptr as *mut __JNI_MyOtherStruct);

    it.b = val as *mut MyStruct;

    ptr as jlong
}

// ========================= Constructors & Destructors =========================

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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_docs_example_complex_MyOtherStruct_jni_1init_1new<
    'local,
>(
    mut env: JNIEnv<'local>,
    _obj: JObject<'local>,
) -> jlong {
    let it = __JNI_MyOtherStruct::new();
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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_docs_example_complex_MyOtherStruct_jni_1free<
    'local,
>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
) {
    let it = Box::from_raw(ptr as *mut __JNI_MyOtherStruct);
    let _ = Box::from_raw(it.b);
}

// ========================= Methods =========================

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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_docs_example_complex_MyOtherStruct_jni_1say_1only<
    'local,
>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
    message: JString<'local>,
) {
    let it = &*(ptr as *mut __JNI_MyOtherStruct);
    let message = env
        .get_string(&message)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    it.__wrapped_say_only(message);
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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_docs_example_complex_MyOtherStruct_jni_1say<
    'local,
>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
    p2: JString<'local>,
) {
    let it = &*(ptr as *mut __JNI_MyOtherStruct);
    let p2 = env.get_string(&p2).unwrap().to_str().unwrap().to_string();

    it.__wrapped_say(p2);
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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_docs_example_complex_MyOtherStruct_jni_1say_1with<
    'local,
>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
    p1: jlong,
    p2: JString<'local>,
) {
    let it = &*(ptr as *mut __JNI_MyOtherStruct);
    let p1 = &*(p1 as *mut MyStruct);
    let p2 = env.get_string(&p2).unwrap().to_str().unwrap().to_string();

    it.__wrapped_say_with(p1.clone(), p2);
}