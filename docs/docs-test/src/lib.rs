pub mod my_struct;
pub mod my_other_struct;

use jni::sys::jlong;

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

#[derive(Debug, Clone)]
pub struct MyStruct {
    pub a: String,
    pub b: i32,
    pub c: f64,
}

#[derive(Debug)]
pub struct MyOtherStruct {
    pub a: String,
    pub b: MyStruct,
}

impl MyStruct {
    pub fn new() -> Self {
        Self {
            a: String::from("Test"),
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
