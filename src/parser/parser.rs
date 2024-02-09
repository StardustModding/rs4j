use super::{bound::BoundExpr, class::ClassExpr, expr::Expr, func::FunctionExpr, ty::TypeExpr};

parser! {
    pub grammar rs4j_parser() for str {
        pub rule class() -> Expr
            = [' ' | '\t' | '\n']* "class" _ name: identifier() _
            generics: ("<" generics: ((_ t: identifier() _ { t }) ** ",") ">" _ { generics })?
            "{" _ stmts: statements() _ "}" _ ";" _
            { Expr::Class(ClassExpr { name: Box::new(name), stmts: Box::new(stmts), generics: Box::new(generics) }) }

        pub rule classes() -> Vec<Expr>
            = c: (class_expr()*) { c }

        pub rule class_expr() -> Expr
            = _ e: class() _ "\n" { e }

        pub rule statements() -> Vec<Expr>
            = s: (statement()*) { s }

        pub rule statement() -> Expr
            = _ e: expression() _ "\n" { e }

        pub rule expression() -> Expr
            = function() / bound() / comment() / { Expr::None }

        pub rule bound() -> Expr
            = [' ' | '\t' | '\n']* _ "bound" _ name: identifier() _ ":"
            _ traits: ((_ tr: _type() _ { tr }) ** "+") _ ";" _
            { Expr::Bound(BoundExpr { name: Box::new(name), traits: Box::new(traits) }) }

        pub rule function() -> Expr
            = [' ' | '\t' | '\n']* _ rust_name: ("[" _ rust_name: identifier() _ "]" _ { rust_name })? _ static_: "static"? _
            _ mut_: "mut"? _
            "fn" _ src: (src: identifier() _ "::" _ {src})? _
            name: identifier() _ "(" args: (
                (
                    _ i: identifier() _ ":" _ borrow: ("&")? _
                    borrow_mut: ("&" _ "mut")? _
                    t: _type() _
                    { (i, t, borrow.is_some(), borrow_mut.is_some()) }) ** ","
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
            = id: identifier() _ generics: ("<" generics: ((_ t: _type() _ { t }) ** ",") ">" _ { generics })?
            { Expr::Type(TypeExpr { id: Box::new(id), generics: Box::new(generics) }) }

        pub rule comment() -> Expr
            = "//" _ data_: ([^ '\n']*) { Expr::None }

        rule _() = quiet! { [' ' | '\t']* }
    }
}

pub use rs4j_parser::*;
