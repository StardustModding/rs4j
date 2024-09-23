//! Internal conversions

/// A trait for objects that convert into [`jobject`]s.
pub trait AsJava<'a> {
    /// Convert into a [`jobject`].
    fn as_java(self, env: jni::JNIEnv<'a>) -> jni::sys::jobject;
}

impl<'a> AsJava<'a> for String {
    fn as_java(self, env: jni::JNIEnv<'a>) -> jni::sys::jobject {
        env.new_string(self).unwrap().as_raw()
    }
}

impl<'a> AsJava<'a> for &str {
    fn as_java(self, env: jni::JNIEnv<'a>) -> jni::sys::jobject {
        env.new_string(self.to_string()).unwrap().as_raw()
    }
}

macro_rules! conversion {
    ($ty: ty => $other: ident) => {
        impl<'a> AsJava<'a> for $ty {
            fn as_java(self, _env: jni::JNIEnv<'a>) -> jni::sys::jobject {
                self as jni::sys::$other as jni::sys::jobject
            }
        }
    };
}

conversion!(u8 => jbyte);
conversion!(u16 => jshort);
conversion!(u32 => jint);
conversion!(u64 => jlong);
conversion!(i8 => jbyte);
conversion!(i16 => jshort);
conversion!(i32 => jint);
conversion!(i64 => jlong);
conversion!(char => jchar);
conversion!(bool => jboolean);

// TODO
// conversion!(f32 => jfloat);
// conversion!(f64 => jdouble);
// conversion!(u128 => jbigint);
// conversion!(i128 => jbigint);
