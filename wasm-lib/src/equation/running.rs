use num_complex::{Complex64, ComplexFloat};

use crate::parsing::{
    ast::{ExprNode, Function},
    operators::{BinOp, UnaryOp},
};

use super::system::{Equation, System};

impl ExprNode {
    pub fn run(&self, args: &[Complex64]) -> Complex64 {
        match self {
            ExprNode::Number(v) => v.into(),
            ExprNode::Var(id) => args[*id as usize],
            ExprNode::BinOp(a, op, b) => {
                let a = a.run(args);
                let b = b.run(args);
                match op {
                    BinOp::Plus => a + b,
                    BinOp::Minus => a - b,
                    BinOp::Mult => a * b,
                    BinOp::Div => a / b,
                    BinOp::Mod => a % b,
                    BinOp::Pow => a.powc(b),
                }
            }
            ExprNode::UnaryOp(op, v) => {
                let v = v.run(args);
                match op {
                    UnaryOp::Plus => v,
                    UnaryOp::Minus => -v,
                }
            }
            ExprNode::E => std::f64::consts::E.into(),
            ExprNode::Pi => std::f64::consts::PI.into(),
            ExprNode::I => Complex64::i(),
            ExprNode::Abs(v) => v.run(args).abs().into(),
            ExprNode::Func(f, v) => {
                let v = v.run(args);
                match f {
                    Function::Sin => v.sin(),
                    Function::Cos => v.cos(),
                    Function::Tan => v.tan(),
                    Function::SinH => v.sinh(),
                    Function::CosH => v.cosh(),
                    Function::TanH => v.tanh(),
                    Function::Asin => v.asin(),
                    Function::Acos => v.acos(),
                    Function::Atan => v.atan(),
                    Function::AsinH => v.asinh(),
                    Function::AcosH => v.acosh(),
                    Function::AtanH => v.atanh(),
                    Function::Ln => v.ln(),
                    Function::Sqrt => v.sqrt(),
                    Function::Cbrt => v.cbrt(),
                    Function::Arg => v.arg().into(),
                }
            }
        }
    }
}

impl Equation {
    pub fn calc(&self, args: &[Complex64]) -> Complex64 {
        self.left.run(args) - self.right.run(args)
    }
}
