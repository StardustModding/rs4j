//! Conversion methods

use crate::if_else;

use super::ty::{Type, TypeKind};

/// Generate conversion code for a variable.
pub fn conversion_method(var: impl AsRef<str>, ty: &Type, mutable: bool) -> Option<String> {
    let var = var.as_ref();
    let mut_ = if_else!(mutable, "mut ", "");

    match ty.kind {
        TypeKind::String => Some(format!(
            "    let {mut_}{} = env.get_string(&{}).unwrap().to_str().unwrap().to_string();",
            var, var
        )),
        TypeKind::Other(_) => Some(format!(
            "    let {} = &{mut_}*({} as *mut {});",
            var,
            var,
            ty.full_type()
        )),
        _ => None,
    }
}
