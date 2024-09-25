//! Macros for rs4j

/// If a value is equal to another, throw an error.
#[macro_export]
macro_rules! equals_throw {
    ($self: ident.$var: ident, $val: expr, $err: expr) => {
        if $self.$var == $val {
            return Err($crate::anyhow::anyhow!($err));
        }
    };
}

/// Basically a ternary
#[macro_export]
macro_rules! if_else {
    ($($cond: expr)+, $($v1: expr)+, $($v2: expr)+) => {
        if $($cond)+ {
            $($v1)+
        } else {
            $($v2)+
        }
    }
}

/// Make a getter
#[macro_export]
macro_rules! getter {
    ($m: ident = $s: ident.$f: ident as $t: ident) => {
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
        pub unsafe extern "system" fn $m<'local>(
            mut env: JNIEnv<'local>,
            class: JClass<'local>,
            ptr: jlong,
        ) -> $t {
            let it: &$s = jlong_to_pointer::<$s>(ptr).as_mut().unwrap();
            it.$f as $t
        }
    };
}

/// Make a setter
#[macro_export]
macro_rules! setter {
    ($m: ident = $ty: ty = $s: ident.$f: ident as $t: ident) => {
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
        pub unsafe extern "system" fn $m<'local>(
            mut env: JNIEnv<'local>,
            class: JClass<'local>,
            ptr: jlong,
            val: $ty,
        ) -> jlong {
            let it: &mut $s = jlong_to_pointer::<$s>(ptr).as_mut().unwrap();

            it.$f = val;

            Box::into_raw(Box::new(it)) as jlong
        }
    };
}
