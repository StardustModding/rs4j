//! The PEG parser.

use crate::class::{
    Class,
    arg::FunctionArg,
    expr::Expr,
    field::Field,
    generic::TypeGeneric,
    method::Method,
    ty::{Type, TypeKind},
};

parser! {
    /// The rs4j parser.
    pub grammar rs4j_parser() for str {
        /// Parse many [`Class`]es.
        pub rule classes() -> Vec<Class>
            = c: (class()*) { c }

        /// Parse a [`Class`].
        pub rule class() -> Class
            = _ e: _class() _ "\n" { e }

            rule _class() -> Class
            = __ _ wrapped: "wrapped"? _
            "class" _ name: _ident() _
            generics: _generics()? _
            real_name: ("=" _ id: _ident() _ generics: _generics()? { (id, generics.unwrap_or_default()) })? _
            "{" _ stmts: stmts() _ "}" _ ";"?
            {
                let class_generics: Vec<TypeGeneric> = generics.unwrap_or_default().iter().map(|v| v.clone().into()).collect::<Vec<_>>();
                let fields = stmts.iter().filter_map(|v| v.get_field()).collect::<Vec<_>>();
                let mut generics = stmts.iter().filter_map(|v| v.get_generic()).collect::<Vec<_>>();
                let methods = stmts.iter().filter_map(|v| v.get_method()).collect::<Vec<_>>();

                for item in class_generics {
                    if generics.iter().find(|v| v.name == item.name).is_none() {
                        generics.push(item);
                    }
                }

                Class {
                    name,
                    real_name,
                    wrapped: wrapped.is_some(),
                    package: String::new(),
                    fields,
                    imports: Class::default_imports(),
                    generics,
                    methods,
                }
            }

        /// Parse multiple [`Expr`]s.
        pub rule stmts() -> Vec<Expr>
            = s: (stmt()*) { s }

        /// Parse a statement ([`Expr`]).
        pub rule stmt() -> Expr
            = _ e: expr() _ "\n" { e }

        /// Parse an [`Expr`].
        pub rule expr() -> Expr = method() / generic() / field() / comment() / { Expr::None }

        /// Parse a [`Method`] as an [`Expr`].
        pub rule generic() -> Expr = val: _generic() { Expr::Generic(val) }

        rule _generic() -> TypeGeneric
            = __ rust_only: ("[" _ "rust" _ "]")? _ free: "free"? _ "bound" _ name: _ident() bounds: (_ ":"
            _ bounds: (ty: _type() ** (_ "+" _) { ty }) { bounds })? _ ";"
            { TypeGeneric { name, bounds: bounds.unwrap_or_default(), rust_only: rust_only.is_some(), free: free.is_some() } }

        /// Parse a [`Field`] as an [`Expr`].
        pub rule field() -> Expr = val: _field() { Expr::Field(val) }

        rule _field() -> Field
            = __ _ rust: "rust"? _ "field" _ name: _ident() _ ":"
            _ ty: _type() _ ";"
            { Field { name, rust: rust.is_some(), ty } }

        /// Parse a [`FunctionArg`].
        pub rule fn_arg() -> FunctionArg
            = _ name: _ident() _ ":" _
            into: "#into"? _
            borrow: "&"? _
            mutable: "mut"? _
            ty: _type() _

            {
                FunctionArg {
                    name,
                    ty,
                    borrow: borrow.is_some(),
                    mutable: mutable.is_some(),
                    into: into.is_some(),
                }
            }

        /// Parse a [`Method`] as an [`Expr`].
        pub rule method() -> Expr = val: _method() { Expr::Method(val) }

        rule _method() -> Method
            = __ _ rust_name: ("[" _ id: _ident() _ "]" { id })?
            _ modifiers: (v: _func_modifier() ** _ { v }) _
            "fn" _ object: (id: _ident() _ "::" _ { id })? _
            name: _ident() _
            "(" _ args: (fn_arg() ** ",") _ ")" _
            ret: ("-" _ ">" _ ty: _type() _ { ty })? _
            ";"

            {
                Method {
                    custom_name: rust_name,
                    args,
                    object,
                    name,
                    ret: ret.unwrap_or_default(),
                    boxed: modifiers.contains(&"boxed".into()),
                    is_consumed: modifiers.contains(&"consumed".into()),
                    is_init: modifiers.contains(&"init".into()),
                    is_mut: modifiers.contains(&"mut".into()),
                    is_optional: modifiers.contains(&"optional".into()),
                    is_static: modifiers.contains(&"static".into()),
                }
            }

        /// Parse a [`Type`].
        pub rule _type() -> Type
            = id: _type_k() _ generics: (_generics())?
            { Type { kind: id, generics } }

        /// Parse a [`TypeKind`].
        pub rule _type_k() -> TypeKind = _uint_k() / _int_k() / _float_k() / _extra_k()

        // Type kinds

        rule _u8_k() -> TypeKind = "u8" { TypeKind::U8 }
        rule _u16_k() -> TypeKind = "u16" { TypeKind::U16 }
        rule _u32_k() -> TypeKind = "u32" { TypeKind::U32 }
        rule _u64_k() -> TypeKind = "u64" { TypeKind::U64 }
        rule _i8_k() -> TypeKind = "i8" { TypeKind::I8 }
        rule _i16_k() -> TypeKind = "i16" { TypeKind::I16 }
        rule _i32_k() -> TypeKind = "i32" { TypeKind::I32 }
        rule _i64_k() -> TypeKind = "i64" { TypeKind::I64 }
        rule _f32_k() -> TypeKind = "f32" { TypeKind::F32 }
        rule _f64_k() -> TypeKind = "f64" { TypeKind::F64 }
        rule _bool_k() -> TypeKind = "bool" { TypeKind::Bool }
        rule _char_k() -> TypeKind = "char" { TypeKind::Char }
        rule _str_k() -> TypeKind = "String" { TypeKind::String }
        rule _void_k() -> TypeKind = "()" { TypeKind::Void }
        rule _other_k() -> TypeKind = id: _ident() { TypeKind::Other(id) }
        rule _uint_k() -> TypeKind = _u8_k() / _u16_k() / _u32_k() / _u64_k()
        rule _int_k() -> TypeKind = _i8_k() / _i16_k() / _i32_k() / _i64_k()
        rule _float_k() -> TypeKind = _f32_k() / _f64_k()
        rule _extra_k() -> TypeKind = _bool_k() / _char_k() / _str_k() / _void_k() / _other_k()

        // Generics

        rule _generics() -> Vec<Type> = "<" generics: ((_ t: _type() _ { t }) ** ",") ">" _ { generics }

        // Function Modifiers

        rule _boxed() -> String = "boxed" { "boxed".into() }
        rule _static() -> String = "static" { "static".into() }
        rule _init() -> String = "init" { "init".into() }
        rule _mut() -> String = "mut" { "mut".into() }
        rule _consumed() -> String = "consumed" { "consumed".into() }
        rule _optional() -> String = "optional" { "optional".into() }

        rule _func_modifier() -> String = _boxed() / _static() / _init() / _mut() / _consumed() / _optional()

        // Utilities

        rule _ident() -> String
            = quiet! {
                n: $(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*)
                { n.to_owned() }
            } / expected!("identifier")

        rule comment() -> Expr = "//" _ [^ '\n']* { Expr::None }
        rule _() = quiet! { [' ' | '\t']* }
        rule __() = quiet! { [' ' | '\t' | '\n']* }
    }
}

pub use rs4j_parser::*;
