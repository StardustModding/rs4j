use jni::{objects::{JClass, JObject, JString}, sys::{jlong, jstring}, JNIEnv};

use crate::{MyOtherStruct, MyStruct};

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct __JNI_MyOtherStruct {
    pub a: String,
    pub b: *mut MyStruct,
}

impl __JNI_MyOtherStruct {
    pub fn new() -> Self {
        Self {
            a: String::new(),
            b: Box::leak(Box::new(MyStruct::new())) as *mut MyStruct,
        }
    }

    pub unsafe fn to_rust(&self) -> MyOtherStruct {
        MyOtherStruct {
            a: self.a.clone(),
            b: (&mut *self.b).clone(),
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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_docs_example_complex_MyOtherStruct_jni_1get_1a<'local>(
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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_docs_example_complex_MyOtherStruct_jni_1get_1b<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
) -> jlong {
    let it = &*(ptr as *mut __JNI_MyOtherStruct);
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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_docs_example_complex_MyOtherStruct_jni_1set_1a<'local>(
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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_docs_example_complex_MyOtherStruct_jni_1set_1b<'local>(
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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_docs_example_complex_MyOtherStruct_jni_1init_1new<'local>(
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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_docs_example_complex_MyOtherStruct_jni_1free<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
) {
    let it = Box::from_raw(ptr as *mut __JNI_MyOtherStruct);
    let _ = Box::from_raw(it.b);
}
