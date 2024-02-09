#[macro_export]
macro_rules! equals_throw {
    ($self: ident.$var: ident, $val: expr, $err: expr) => {
        if $self.$var == $val {
            return Err($crate::anyhow::anyhow!($err));
        }
    };
}
