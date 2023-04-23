#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Plus,
    Minus,
    Mult,
    Div,
    Pow,
    Mod,
}
#[derive(Debug, Clone, Copy)]
pub enum Function {
    Sin,
    Cos,
    Tan,
}

#[derive(Debug, Clone)]
pub enum ExprNode {
    Number(f64),
    Var(u16),

    BinOp(Box<ExprNode>, BinOp, Box<ExprNode>),
    Negate(Box<ExprNode>),

    E,
    Pi,
    I,

    Func(Function, Vec<ExprNode>),
}
