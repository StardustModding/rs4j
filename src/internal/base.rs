//! JNI functions for the NativeTools class

use jni::{
    objects::{JClass, JString},
    sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jobject, jshort, jstring},
    JNIEnv,
};

use super::include::object_to_jobject;

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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_util_NativeTools_getString<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    let ptr = ptr as *const String;

    env.new_string(std::ptr::read(ptr)).unwrap().as_raw()
}

macro_rules! basic_method {
    ($name: ident = $ty: ident ($t2: ident)) => {
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
        pub unsafe extern "system" fn $name<'local>(
            mut env: JNIEnv<'local>,
            class: JClass<'local>,
            ptr: jlong,
        ) -> $ty {
            let ptr = ptr as *const $t2;

            std::ptr::read(ptr) as $ty
        }
    };
}

basic_method!(Java_org_stardustmodding_rs4j_util_NativeTools_getBool = jboolean(bool));
basic_method!(Java_org_stardustmodding_rs4j_util_NativeTools_getByte = jbyte(i8));
basic_method!(Java_org_stardustmodding_rs4j_util_NativeTools_getShort = jshort(i16));
basic_method!(Java_org_stardustmodding_rs4j_util_NativeTools_getInt = jint(i32));
basic_method!(Java_org_stardustmodding_rs4j_util_NativeTools_getLong = jlong(i64));
basic_method!(Java_org_stardustmodding_rs4j_util_NativeTools_getChar = jchar(char));
basic_method!(Java_org_stardustmodding_rs4j_util_NativeTools_getFloat = jfloat(f32));
basic_method!(Java_org_stardustmodding_rs4j_util_NativeTools_getDouble = jdouble(f64));

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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_util_NativeTools_getObjectJni<
    'local,
>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
    cls: JString<'local>,
) -> jobject {
    let cls = env.get_string(&cls).unwrap();
    let cls = cls.to_str().unwrap();
    object_to_jobject(env, ptr, cls.to_string())
}
