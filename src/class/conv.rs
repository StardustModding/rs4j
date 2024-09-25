//! Conversion methods

use crate::if_else;

use super::ty::TypeKind;

/// Generate conversion code for a variable.
pub fn conversion_method(var: impl AsRef<str>, ty: &TypeKind, mutable: bool) -> Option<String> {
    let var = var.as_ref();
    let mut_ = if_else!(mutable, "mut ", "");

    match ty {
        TypeKind::String => Some(format!(
            "let {mut_}{} = env.get_string(&{}).unwrap().to_str().unwrap().to_string();",
            var, var
        )),
        TypeKind::Other(v) => Some(format!("let {} = &{mut_}*({} as *mut {});", var, var, v)),
        _ => None,
    }
}
