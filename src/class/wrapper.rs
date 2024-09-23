//! Wrapper methods.

use crate::{class::ty::TypeKind, if_else};

use super::{arg::FunctionArg, ty::Type};

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

    /// Is this a static method?
    pub is_static: bool,
}

impl WrapperMethod {
    /// Generate Java code for this method.
    pub fn java_code(&self) -> String {
        let static_code = if_else!(self.is_static, " static", "");
        let return_code = if_else!(self.ret.kind == TypeKind::Void, "return ", "");
        let native = &self.calls;
        let name = &self.name;
        // TODO: Resolve & dedupe generics for args
        let generics = if_else!(self.ret.generics.is_some(), format!("<{}>", self.ret.get_generics_java()), "".into());
        let ret = self.ret.full_type_java();
        let convert = "todo";
        let mut args = Vec::new();
        let mut args_nt = Vec::new();

        for arg in &self.args {
            args.push(format!("{} {}", arg.ty.full_type_java(), arg.name));
            args_nt.push(arg.name.clone());
        }

        let args = args.join(", ");
        let args_nt = args_nt.join(", ");

        if self.ret.kind.is_primitive() {
            format!("    public{static_code}{generics} {ret} {name}({args}) {{\n        return {native}({args_nt});\n    }}")
        } else {
            format!("    public{static_code}{generics} {ret} {name}({args}) {{\n        long val = {native}({args_nt});\n        {return_code}{convert}(val);\n    }}")
        }
    }

    /// Generate Rust code for this method.
    pub fn rust_code(&self) -> String {
        let mut args = Vec::new();
        let mut args_nt = Vec::new();

        for arg in &self.args {
            let mut extra = "";
            let mut pre = "";

            if arg.borrow && arg.mutable {
                extra = "&mut ";
            } else if arg.borrow && !arg.mutable {
                extra = "&";
            } else if !arg.borrow && arg.mutable {
                pre = "mut ";
            }

            args.push(format!("{}{}: {}{}", pre, arg.name, extra, arg.ty.full_type()));
            args_nt.push(arg.name.clone());
        }

        todo!("How to handle custom types? Potentially make a transformer on both ends?")
    }
}
