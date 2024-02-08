#[allow(unused_imports)]
use jni::sys::{
    jarray, jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jobject, jshort, jstring,
    jvalue,
};

use jni::{
    objects::{JObject, JValueGen},
    JNIEnv,
};

#[cfg(target_pointer_width = "32")]
pub unsafe fn jlong_to_pointer<T>(val: jlong) -> *mut T {
    (val as u32) as *mut T
}

#[cfg(target_pointer_width = "64")]
pub unsafe fn jlong_to_pointer<T>(val: jlong) -> *mut T {
    val as *mut T
}

#[allow(dead_code)]
fn object_to_jobject<T>(env: *mut JNIEnv, obj: T, jcls: String) -> jobject {
    let jobj: JObject = unsafe { (*env).alloc_object(jcls).unwrap() };

    assert!(!jobj.is_null(), "object_to_jobject: AllocObject failed");

    let ret: jlong = Box::into_raw(Box::new(obj)) as jlong;

    unsafe {
        let res = (*env).set_field(&jobj, "__pointer", "Long", JValueGen::Long(ret));

        if (*env).exception_check().unwrap() || res.is_err() {
            panic!("object_to_jobject: Can not set mNativeObj field: catch exception");
        }
    }

    jobj.as_raw()
}
