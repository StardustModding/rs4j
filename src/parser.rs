use crate::types::{IntoJavaType, RustTypes};
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct FunctionExpr {
    /// The original struct the function belongs to.
    pub source: Box<Option<Expr>>,

    /// The function name.
    pub name: Box<Expr>,

    /// The native function name in Rust.
    pub rust_name: Box<Option<Expr>>,

    /// The function arguments.
    pub args: Box<Vec<(Expr, Expr)>>,

    /// The return type.
    pub ret: Box<Option<Expr>>,

    /// Is it static?
    pub is_static: bool,

    /// Does it need &mut self?
    pub is_mut: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct ClassExpr {
    /// The name of the class.
    pub name: Box<Expr>,

    /// The statements in the class.
    pub stmts: Box<Vec<Expr>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct TypeExpr {
    /// The type's ID.
    pub id: Box<Expr>,

    /// The type's generics.
    pub generics: Box<Option<Vec<Expr>>>,
}

impl TypeExpr {
    pub fn as_java(&self) -> Result<String> {
        let ident = self.id.ident_strict()?;
        let java_type = RustTypes::from(ident.as_str()).into_java_type();

        if let Some(generics) = *self.generics.clone() {
            let generics = generics
                .iter()
                .map(|v| {
                    v.ident_strict_java()
                        .unwrap_or(v.get_type().unwrap().as_java().unwrap())
                })
                .collect::<Vec<String>>()
                .join(", ");

            Ok(format!("{}<{}>", java_type, generics))
        } else {
            Ok(java_type)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum Expr {
    Identifier(String),
    Function(FunctionExpr),
    Class(ClassExpr),
    Type(TypeExpr),

    #[default]
    None,
}

impl Expr {
    pub fn ident(&self) -> Result<String> {
        if let Self::Identifier(val) = self {
            Ok(val.clone())
        } else if let Self::Type(val) = self {
            if let Some(generics) = *val.generics.clone() {
                let generics = generics
                    .iter()
                    .map(|v| v.ident().unwrap())
                    .collect::<Vec<String>>()
                    .join(", ");

                Ok(format!("{}<{}>", val.id.ident()?, generics))
            } else {
                Ok(val.id.ident()?)
            }
        } else {
            Err(anyhow!("Expected Self::Identifier(_), got {:?}", self))
        }
    }

    pub fn ident_strict(&self) -> Result<String> {
        if let Self::Identifier(val) = self {
            Ok(val.clone())
        } else {
            Err(anyhow!("Expected Self::Identifier(_), got {:?}", self))
        }
    }

    pub fn ident_strict_java(&self) -> Result<String> {
        if let Self::Identifier(val) = self {
            Ok(RustTypes::from(val.clone().as_str()).into_java_type())
        } else {
            Err(anyhow!("Expected Self::Identifier(_), got {:?}", self))
        }
    }

    pub fn get_type(&self) -> Result<TypeExpr> {
        if let Self::Type(val) = self {
            Ok(val.clone())
        } else {
            Err(anyhow!("Expected Self::Type(_), got {:?}", self))
        }
    }
}

parser! {
    pub grammar rs4j_parser() for str {
        pub rule class() -> Expr
            = [' ' | '\t' | '\n']* "class" _ name: identifier() _
            "{" _ stmts: statements() _ "}" _ ";" _
            { Expr::Class(ClassExpr { name: Box::new(name), stmts: Box::new(stmts) }) }

        pub rule classes() -> Vec<Expr>
            = c: (class_expr()*) { c }

        pub rule class_expr() -> Expr
            = _ e: class() _ "\n" { e }

        pub rule statements() -> Vec<Expr>
            = s: (statement()*) { s }

        pub rule statement() -> Expr
            = _ e: expression() _ "\n" { e }

        pub rule expression() -> Expr
            = function() / comment() / { Expr::None }

        pub rule function() -> Expr
            = [' ' | '\t' | '\n']* _ rust_name: ("[" _ rust_name: identifier() _ "]" _ { rust_name })? _ static_: "static"? _
            _ mut_: "mut"? _
            "fn" _ src: (src: identifier() _ "::" _ {src})? _
            name: identifier() _ "(" args: (
                (_ i: identifier() _ ":" _ t: _type() _ { (i, t) }) ** ","
            ) ")" _
            ret: ("-" _ ">" _ ret: _type() _ {ret})? _
            ";" _

            {
                Expr::Function(FunctionExpr {
                    is_static: static_.is_some(),
                    is_mut: mut_.is_some(),
                    rust_name: Box::new(rust_name),
                    name: Box::new(name),
                    source: Box::new(src),
                    ret: Box::new(ret),
                    args: Box::new(args),
                })
            }

        pub rule identifier() -> Expr
            = quiet! {
                n: $(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*)
                { Expr::Identifier(n.to_owned()) }
            } / expected!("identifier")

        pub rule _type() -> Expr
            = id: identifier() generics: ("<" generics: ((_ t: _type() _ { t }) ** ",") ">" _ { generics })?
            { Expr::Type(TypeExpr { id: Box::new(id), generics: Box::new(generics) }) }
        
        pub rule comment() -> Expr
            = "//" _ data_: ([^ '\n']*) { Expr::None }

        rule _() = quiet! { [' ' | '\t']* }
    }
}

pub use rs4j_parser::*;
