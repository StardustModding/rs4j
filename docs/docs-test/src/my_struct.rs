use jni::{objects::{JClass, JObject, JString}, sys::{jdouble, jint, jlong, jstring}, JNIEnv};
use rs4j::{getter, setter};
use crate::{jlong_to_pointer, MyStruct};

getter!(Java_org_stardustmodding_rs4j_docs_example_complex_MyStruct_jni_1get_1b = MyStruct.b as jint);
getter!(Java_org_stardustmodding_rs4j_docs_example_complex_MyStruct_jni_1get_1c = MyStruct.c as jdouble);
setter!(Java_org_stardustmodding_rs4j_docs_example_complex_MyStruct_jni_1set_1b = jint = MyStruct.b as i32);
setter!(Java_org_stardustmodding_rs4j_docs_example_complex_MyStruct_jni_1set_1c = jdouble = MyStruct.c as f64);

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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_docs_example_complex_MyStruct_jni_1get_1a<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    let it = &*(ptr as *mut MyStruct);
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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_docs_example_complex_MyStruct_jni_1set_1a<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
    val: JString<'local>,
) -> jlong {
    let it = &mut *(ptr as *mut MyStruct);
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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_docs_example_complex_MyStruct_jni_1init_1new<'local>(
    mut env: JNIEnv<'local>,
    _obj: JObject<'local>,
) -> jlong {
    let it = MyStruct::new();
    (Box::leak(Box::new(it)) as *mut MyStruct) as jlong
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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_docs_example_complex_MyStruct_jni_1free<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
) {
    let _ = Box::from_raw(ptr as *mut MyStruct);
}
