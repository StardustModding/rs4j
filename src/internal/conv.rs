//! Internal conversions

/// A trait for objects that convert into Java types.
pub trait AsJava<'a> {
    /// Get the function associated with getting this type.
    fn java_fn(&self) -> String;
}

/// A trait for objects that convert into a Java pointer.
pub trait AsJavaPtr {
    /// Convert into a pointer.
    fn as_java_ptr(self) -> *const Self;
}

impl<T> AsJavaPtr for T {
    fn as_java_ptr(self) -> *const Self {
        Box::into_raw(Box::new(self))
    }
}

impl<'a> AsJava<'a> for String {
    fn java_fn(&self) -> String {
        "NativeTools.getString".into()
    }
}

macro_rules! conversion {
    ($ty: ty => $other: ident: $func: ident) => {
        impl<'a> AsJava<'a> for $ty {
            fn java_fn(&self) -> String {
                format!("NativeTools.{}", stringify!($func))
            }
        }
    };
}

conversion!(u8 => jbyte: getByte);
conversion!(u16 => jshort: getShort);
conversion!(u32 => jint: getInt);
conversion!(u64 => jlong: getLong);
conversion!(i8 => jbyte: getByte);
conversion!(i16 => jshort: getShort);
conversion!(i32 => jint: getInt);
conversion!(i64 => jlong: getLong);
conversion!(char => jchar: getChar);
conversion!(bool => jboolean: getBool);
conversion!(f32 => jfloat: getFloat);
conversion!(f64 => jdouble: getDouble);

// TODO
// conversion!(u128 => jbigint);
// conversion!(i128 => jbigint);
