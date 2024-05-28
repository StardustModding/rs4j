use super::{bound::BoundExpr, class::ClassExpr, expr::Expr, func::FunctionExpr, ty::TypeExpr};

parser! {
    pub grammar rs4j_parser() for str {
        pub rule class() -> Expr
            = [' ' | '\t' | '\n']* "class" _ name: identifier() _
            generics: (generics())?
            real_name: (
                "=" _ real: identifier() _
                real_generics: (generics())? _
                { (real, real_generics) }
            )?
            "{" _ stmts: statements() _ "}" _ ";" _
            { Expr::Class(ClassExpr { name: Box::new(name.clone()), real_name: Box::new(real_name.unwrap_or((name, generics.clone()))), stmts: Box::new(stmts), generics: Box::new(generics) }) }

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

        pub rule generics() -> Vec<Expr>
            = "<" generics: ((_ t: _type() _ { t }) ** ",") ">" _ { generics }
        
        pub rule generics_with_bounds() -> Vec<(Expr, Option<Vec<Expr>>)>
            = "<" generics: ((_ t: _type() _ b: (":" _ traits: (_type() ** ",") _ { traits })? _ { (t, b) }) ** ",") ">" _ { generics }

        pub rule bound() -> Expr
            = [' ' | '\t' | '\n']* _ "bound" _ name: identifier() _ ":"
            _ traits: ([^';']+) _ ";" _
            { Expr::Bound(BoundExpr { name: Box::new(name), traits: String::from_iter(traits) }) }

        pub rule function() -> Expr
            = [' ' | '\t' | '\n']* _ rust_name: ("[" _ rust_name: identifier() _ "]" _ { rust_name })?
            _ static_: "static"? _
            _ mut_: "mut"? _
            _ consumed: "consumed"? _
            _ optional: "optional"? _
            "fn" _ src: (src: identifier() _ "::" _ {src})? _
            name: identifier() _
            generics: (generics_with_bounds())? _
            "(" args: (
                (
                    i: identifier() _ ":" _
                    into: ("#into")? _
                    borrow: ("&")? _
                    borrow_mut: ("mut")? _
                    t: _type() _
                    { (i, t, borrow.is_some(), borrow.is_some() && borrow_mut.is_some(), false) }
                ) ** ","
            ) ")" _
            ret: ("-" _ ">" _ ret: _type() _ {ret})? _
            ";" _

            {
                Expr::Function(FunctionExpr {
                    generics: Box::new(generics.unwrap_or_default()),
                    is_static: static_.is_some(),
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
