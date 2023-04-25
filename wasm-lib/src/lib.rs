#![deny(unused_must_use)]

mod equation;
mod utils;

use std::collections::HashMap;

use equation::system::{Equation, System};
use num_complex::Complex64;
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
pub fn solve(
    eqs: JsValue,
    iter: usize,
    initial_values: &[f64],
) -> Result<Option<Vec<JsValue>>, PositionedError> {
    let example: Vec<(String, String)> = serde_wasm_bindgen::from_value(eqs).unwrap();
    let (system, names) = get_eqs(&example)?;

    let solution = system.solve(iter, initial_values.iter().map(|f| Complex64::new(*f, *f)));
    match solution {
        Some(sol) => {
            return Ok(Some(
                names
                    .iter()
                    .zip(sol)
                    .map(|(name, sol)| {
                        serde_wasm_bindgen::to_value(&(name, (sol.re, sol.im))).unwrap()
                    })
                    .collect(),
            ))
        }
        None => Ok(None),
    }
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

pub fn get_eqs(eqs: &[(String, String)]) -> Result<(System, Vec<String>), PositionedError> {
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

    let mut name_vec = vec![String::new(); name_map.len()];
    for (n, id) in name_map {
        name_vec[id as usize] = n
    }

    Ok((
        System {
            eqs: out,
            var_amount: name_vec.len(),
        },
        name_vec,
    ))
}
