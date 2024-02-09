use super::expr::Expr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct FunctionExpr {
    /// The original struct the function belongs to.
    pub source: Box<Option<Expr>>,

    /// The function name.
    pub name: Box<Expr>,

    /// The native function name in Rust.
    pub rust_name: Box<Option<Expr>>,

    /// The function arguments. Format: (name, type, borrow, borrow_mut)
    pub args: Box<Vec<(Expr, Expr, bool, bool)>>,

    /// The return type.
    pub ret: Box<Option<Expr>>,

    /// Is it static?
    pub is_static: bool,

    /// Does it need &mut self?
    pub is_mut: bool,

    /// Does it return an option?
    pub is_optional: bool,

    /// Does it consume self?
    pub is_consumed: bool,
}
