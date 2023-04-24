use super::operators::BinOp;

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
