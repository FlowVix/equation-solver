use super::operators::{BinOp, UnaryOp};
use wasm_bindgen::prelude::*;

macro_rules! functions {
    (
        $(
            $name:ident: $str:literal,
        )*
    ) => {
        #[derive(Debug, Clone, Copy)]
        pub enum Function {
            $(
                $name,
            )*
        }

        impl Function {
            pub fn from_str(s: &str) -> Option<Self> {
                match s {
                    $(
                        $str => Some(Self::$name),
                    )*
                    _ => None
                }
            }
        }

        #[wasm_bindgen]
        pub fn is_not_var(s: &str) -> bool {
            match s {
                "e" | "pi" | "i" $(| $str)* => true,
                _ => false,
            }
        }

        #[wasm_bindgen]
        pub fn funcs() -> String {
            [$($str,)*].join(", ")
        }
    };
}

functions! {
    Sin: "sin",
    Cos: "cos",
    Tan: "tan",
    SinH: "sinh",
    CosH: "cosh",
    TanH: "tanh",

    Asin: "asin",
    Acos: "acos",
    Atan: "atan",
    AsinH: "asinh",
    AcosH: "acosh",
    AtanH: "atanh",

    Sqrt: "sqrt",
    Cbrt: "cbrt",

    Ln: "ln",

    Arg: "arg",
    Re: "re",
    Im: "im",

    Conj: "conj",
}

#[derive(Debug, Clone)]
pub enum ExprNode {
    Number(f64),
    Var(u16),

    BinOp(Box<ExprNode>, BinOp, Box<ExprNode>),
    UnaryOp(UnaryOp, Box<ExprNode>),

    E,
    Pi,
    I,

    Abs(Box<ExprNode>),

    Func(Function, Box<ExprNode>),
}
