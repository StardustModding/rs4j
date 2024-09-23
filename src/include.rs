//! Utils for Java bindings

use jni::{
    sys::{jlong, jobject},
    JNIEnv,
};

use crate::conv::AsJava;

/// Convert a [`jlong`] to a `*mut T`
#[cfg(target_pointer_width = "32")]
pub unsafe fn jlong_to_pointer<T>(val: jlong) -> *mut T {
    (val as u32) as *mut T
}

/// Convert a [`jlong`] to a `*mut T`
#[cfg(target_pointer_width = "64")]
pub unsafe fn jlong_to_pointer<T>(val: jlong) -> *mut T {
    val as *mut T
}

// /// Convert an object (`T`) to a [`jobject`]
// #[allow(dead_code)]
// pub fn object_to_jobject<T>(mut env: JNIEnv, obj: T, jcls: String) -> jobject {
//     let jobj = env.alloc_object(jcls).unwrap();

//     assert!(!jobj.is_null(), "object_to_jobject: AllocObject failed");

//     let ret = Box::into_raw(Box::new(obj)) as jlong;
//     let res = env.set_field(&jobj, "__pointer", "Long", JValueGen::Long(ret));

//     if env.exception_check().unwrap() || res.is_err() {
//         panic!("object_to_jobject: Can not set mNativeObj field: catch exception");
//     }

//     jobj.as_raw()
// }

/// Convert an object (`T`) to a [`jobject`]
#[allow(dead_code)]
pub fn object_to_jobject<'a, T: AsJava<'a>>(env: JNIEnv<'a>, obj: T, _jcls: String) -> jobject {
    obj.as_java(env)
}
