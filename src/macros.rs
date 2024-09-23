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
