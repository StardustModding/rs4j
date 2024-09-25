use rs4j::prelude::*;

#[allow(non_camel_case_types)]
pub struct __JNI_HelloWorld {

}

impl __JNI_HelloWorld {    pub unsafe fn to_rust(&self) -> HelloWorld {
        HelloWorld {

        }
    }

    pub fn __wrapped_hello() -> String {
        HelloWorld::hello()
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
pub unsafe extern "system" fn Java_com_example_HelloWorld_jni_1hello<'local>(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jstring {
    

    env.new_string(__JNI_HelloWorld::__wrapped_hello()).unwrap().as_raw()
}

