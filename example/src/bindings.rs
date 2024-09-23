use rs4j::prelude::*;

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_example_HelloWorld_jni_1hello<'local>(
    mut env: JNIEnv<'local>,
    class: objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, HelloWorld::hello(), "com/example/HelloWorld".to_string())
}

