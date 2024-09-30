//! Wrapper methods.

use convert_case::{Case, Casing};

use super::{ctx::ClassCtx, method::Method, ty::TypeKind};
use crate::if_else;

impl Method {
    /// Generate Java code for this method.
    pub fn wrapper_java_code(&self, cx: &ClassCtx) -> String {
        let static_code = if_else!(self.is_static, " static", "");
        let return_code = if_else!(self.ret.kind != TypeKind::Void, "return ", "");
        let native = &self.calls();
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
