#![deny(unused_must_use)]

mod utils;

use std::collections::HashMap;

use parsing::{ast::ExprNode, parser::Parser};
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod parsing;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn solve(eqs: JsValue) -> Result<String, PositionedError> {
    let example: Vec<(String, String)> = serde_wasm_bindgen::from_value(eqs).unwrap();
    let parsed = get_eqs(&example)?;
    Ok(format!("{:#?}", parsed))
}

#[derive(Debug)]
pub struct System {
    eqs: Vec<Equation>,
    name_map: HashMap<String, u16>,
}

#[derive(Debug)]
pub struct Equation {
    left: ExprNode,
    right: ExprNode,
}
#[wasm_bindgen]
pub struct PositionedError {
    msg: String,
    pub eq: usize,
    pub second: bool,
}
#[wasm_bindgen]
impl PositionedError {
    #[wasm_bindgen(getter = msg)]
    pub fn msg(&self) -> String {
        self.msg.clone()
    }
}

pub fn get_eqs(eqs: &[(String, String)]) -> Result<System, PositionedError> {
    let mut out = vec![];
    let mut name_map = HashMap::new();
    for (i, (a, b)) in eqs.iter().enumerate() {
        let mut parser = Parser::new(a, &mut name_map);
        let left = parser.parse().map_err(|s| PositionedError {
            msg: s,
            eq: i,
            second: false,
        })?;
        let mut parser = Parser::new(b, &mut name_map);
        let right = parser.parse().map_err(|s| PositionedError {
            msg: s,
            eq: i,
            second: true,
        })?;
        out.push(Equation { left, right })
    }
    Ok(System { eqs: out, name_map })
}
