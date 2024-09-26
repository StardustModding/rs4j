//! The PEG parser.

use super::{
    bound::BoundExpr, class::ClassExpr, expr::Expr, field::FieldExpr, func::FunctionExpr,
    ty::TypeExpr,
};

parser! {
    /// The rs4j parser.
    pub grammar rs4j_parser() for str {
        /// Parse a [`ClassExpr`].
        pub rule class() -> Expr
            = [' ' | '\t' | '\n']*
            _ w: "wrapped"? _
            "class" _ name: identifier() _
            generics: (generics())?
            real_name: (
                "=" _ real: identifier() _
                real_generics: (generics())? _
                { (real, real_generics) }
            )?
            "{" _ stmts: statements() _ "}" _ ";" _
            { Expr::Class(ClassExpr { name: Box::new(name.clone()), wrapped: w.is_some(), real_name: Box::new(real_name.unwrap_or((name, generics.clone()))), stmts: Box::new(stmts), generics }) }

        /// Parse many [`ClassExpr`]s.
        pub rule classes() -> Vec<Expr>
            = c: (class_expr()*) { c }

        /// Parse a single [`ClassExpr`].
        pub rule class_expr() -> Expr
            = _ e: class() _ "\n" { e }

        /// Parse [`Expr`]s.
        pub rule statements() -> Vec<Expr>
            = s: (statement()*) { s }

        /// Parse an [`Expr`].
        pub rule statement() -> Expr
            = _ e: expression() _ "\n" { e }

        /// Parse an [`Expr`].
        pub rule expression() -> Expr
            = function() / bound() / field() / comment() / { Expr::None }

        /// Parse generics.
        pub rule generics() -> Vec<TypeExpr>
            = "<" generics: ((_ t: _type() _ { t }) ** ",") ">" _ { generics }

        /// Parse generics with type bounds.
        pub rule generics_with_bounds() -> Vec<(TypeExpr, Option<Vec<TypeExpr>>)>
            = "<" generics: ((_ t: _type() _ b: (":" _ traits: (_type() ** ",") _ { traits })? _ { (t, b) }) ** ",") ">" _ { generics }

        /// Parse a [`BoundExpr`].
        pub rule bound() -> Expr // TODO!
            = [' ' | '\t' | '\n']* _ "bound" _ name: identifier() _ ":"
            _ traits: ([^';']+) _ ";" _
            { Expr::Bound(BoundExpr { name: Box::new(name), traits: String::from_iter(traits) }) }

        /// Parse a [`FieldExpr`].
        pub rule field() -> Expr
            = [' ' | '\t' | '\n']* _ rs: ("[" _ "rust" _ "]")? _ "field" _ name: identifier() _ ":"
            _ ty: (_type()) _ ";" _
            { Expr::Field(FieldExpr { name: Box::new(name), ty, rust_only: rs.is_some() }) }

        /// Parse a function argument.
        pub rule fn_arg() -> (Expr, TypeExpr, bool, bool, bool)
            =  _ i: identifier() _ ":" _
            into: ("#into")? _
            borrow: ("&")? _
            mut_: ("mut")? _
            t: _type() _
            { (i, t, borrow.is_some(), borrow.is_some() && mut_.is_some(), into.is_some()) }

        /// Parse a [`FunctionExpr`].
        pub rule function() -> Expr
            = [' ' | '\t' | '\n']* _ rust_name: ("[" _ rust_name: identifier() _ "]" _ { rust_name })?
            _ boxed: "boxed"? _
            _ static_: "static"? _
            _ init: "init"? _
            _ mut_: "mut"? _
            _ consumed: "consumed"? _
            _ optional: "optional"? _
            "fn" _ src: (src: identifier() _ "::" _ {src})? _
            name: identifier() _
            generics: (g: generics_with_bounds() {g})? _
            "(" args: (fn_arg() ** ",") ")" _
            ret: ("-" _ ">" _ ret: _type() _ {ret})? _
            ";" _

            {
                Expr::Function(FunctionExpr {
                    generics: generics.unwrap_or_default(),
                    boxed: boxed.is_some(),
                    is_static: static_.is_some(),
                    is_init: init.is_some(),
                    is_mut: mut_.is_some(),
                    is_optional: optional.is_some(),
                    is_consumed: consumed.is_some(),
                    rust_name: Box::new(rust_name),
                    name: Box::new(name),
                    source: Box::new(src),
                    ret: Box::new(ret),
                    args: Box::new(args),
                })
            }

        /// Parse an identifier.
        pub rule identifier() -> Expr
            = quiet! {
                n: $(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*)
                { Expr::Identifier(n.to_owned()) }
            } / expected!("identifier")

        /// Parse a [`TypeExpr`].
        pub rule _type() -> TypeExpr
            = id: identifier() _ generics: ("<" generics: ((_ t: _type() _ { t }) ** ",") ">" _ { generics })?
            { TypeExpr { id: Box::new(id), generics: Box::new(generics) } }

        /// Parse (and ignore) a comment.
        pub rule comment() -> Expr
            = "//" _ data_: ([^ '\n']*) { Expr::None }

        rule _() = quiet! { [' ' | '\t']* }
    }
}

pub use rs4j_parser::*;
