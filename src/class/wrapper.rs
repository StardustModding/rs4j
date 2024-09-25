//! Wrapper methods.

use convert_case::{Case, Casing};

use crate::{class::ty::TypeKind, if_else, parser::func::FunctionExpr};

use super::{arg::FunctionArg, ctx::ClassCtx, ty::Type};

/// A method for the user to call that wraps a native method.
pub struct WrapperMethod {
    /// The name of this method.
    pub name: String,

    /// The name of the native method this calls.
    pub calls: String,

    /// This method's arguments.
    pub args: Vec<FunctionArg>,

    /// The return [`Type`] of this method.
    pub ret: Type,

    /// Does this modify the object?
    pub is_mut: bool,

    /// Is this a static method?
    pub is_static: bool,

    /// Is this a constructor?
    pub is_init: bool,
}

impl WrapperMethod {
    /// Generate Java code for this method.
    pub fn java_code(&self, cx: &ClassCtx) -> String {
        let static_code = if_else!(self.is_static, " static", "");
        let return_code = if_else!(self.ret.kind != TypeKind::Void, "return ", "");
        let native = &self.calls;
        let name = &self.name;
        let class = &cx.name;
        // TODO: Resolve & dedupe generics for args
        let generics = if_else!(
            self.ret.generics.is_some(),
            format!("<{}>", self.ret.get_generics_java()),
            "".into()
        );
        let ret = self.ret.full_type_java();
        let convert = self.ret.convert_func();
        let mut args = Vec::new();
        let mut args_nt = Vec::new();

        if !self.is_static {
            args_nt.push("__ptr".into());
        }

        for arg in &self.args {
            args.push(format!("{} {}", arg.ty.full_type_java(), arg.name));
            args_nt.push(arg.java_name());
        }

        let args = args.join(", ");
        let args_nt = args_nt.join(", ");

        if self.is_init {
            return format!(
                "    public {class}({args}) {{\n        __ptr = jni_init_{name}({args_nt});\n    }}"
            );
        }

        let c_name = name.to_case(Case::Camel);

        if self.is_mut {
            if self.ret.kind.is_primitive() {
                if self.ret.kind == TypeKind::Void {
                    format!("    public{static_code}{generics} {ret} {c_name}({args}) {{\n        {native}({args_nt});\n        if (__parent != null) {{\n            __parent.updateField(__parentField, __ptr);\n        }}\n    }}")
                } else {
                    format!("    public{static_code}{generics} {ret} {c_name}({args}) {{\n        {ret} val = {native}({args_nt});\n        if (__parent != null) {{\n            __parent.updateField(__parentField, __ptr);\n        }}\n        return val;\n    }}")
                }
            } else {
                format!("    public{static_code}{generics} {ret} {c_name}({args}) {{\n        long val = {native}({args_nt});        if (__parent != null) {{\n            __parent.updateField(__parentField, __ptr);\n        }}\n        {return_code}{convert}(val);\n    }}")
            }
        } else {
            if self.ret.kind.is_primitive() {
                format!("    public{static_code}{generics} {ret} {c_name}({args}) {{\n        {return_code}{native}({args_nt});\n    }}")
            } else {
                format!("    public{static_code}{generics} {ret} {c_name}({args}) {{\n        long val = {native}({args_nt});\n        {return_code}{convert}(val);\n    }}")
            }
        }
    }
}

impl From<FunctionExpr> for WrapperMethod {
    fn from(value: FunctionExpr) -> Self {
        Self {
            name: value.name.clone().ident_strict().unwrap(),
            calls: format!(
                "jni_{}",
                value
                    .rust_name
                    .map(|v| v.ident_strict().unwrap())
                    .unwrap_or(value.name.ident_strict().unwrap())
            ),
            args: value
                .args
                .iter()
                .cloned()
                .map(|v| v.into())
                .collect::<Vec<_>>(),
            is_init: value.is_init,
            is_mut: value.is_mut,
            is_static: value.is_static,
            ret: value.ret.map(|v| v.into()).unwrap_or_default(),
        }
    }
}
