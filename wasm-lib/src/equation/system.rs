use std::{collections::HashMap, time::Instant};

use nalgebra::{DMatrix, DVector};
use num_complex::Complex64;

use crate::{log, parsing::ast::ExprNode};

#[derive(Debug)]
pub struct System {
    pub eqs: Vec<Equation>,
    pub var_amount: usize,
}

#[derive(Debug)]
pub struct Equation {
    pub left: ExprNode,
    pub right: ExprNode,
}
const DELTA: f64 = 0.000001;

impl System {
    pub fn jacobian(&self, args: &[Complex64]) -> DMatrix<Complex64> {
        let mut shifted_args = args.iter().map(|_| args.to_vec()).collect::<Vec<_>>();

        #[allow(clippy::needless_range_loop)]
        for i in 0..self.var_amount {
            shifted_args[i][i] += Complex64::new(DELTA, DELTA);
        }

        DMatrix::from_iterator(
            self.eqs.len(),
            self.var_amount,
            (0..self.var_amount).flat_map(|i| {
                let shifted_args: &[Complex64] = &shifted_args[i];
                self.eqs
                    .iter()
                    .map(move |eq| (eq.calc(shifted_args) - eq.calc(args)) / DELTA)
            }),
        )
    }
    pub fn run(&self, args: &[Complex64]) -> DVector<Complex64> {
        DVector::from_iterator(self.eqs.len(), self.eqs.iter().map(|eq| eq.calc(args)))
    }
    pub fn solve<R: Iterator<Item = Complex64>>(
        &self,
        iter: usize,
        rand: R,
    ) -> Option<Vec<Complex64>> {
        let mut solution: DVector<Complex64> = DVector::from_iterator(self.var_amount, rand);

        for _ in 0..iter {
            let arg_array: Vec<Complex64> = solution.iter().copied().collect();

            let j = self.jacobian(&arg_array);
            let j_t = j.transpose();
            let j_i = (j_t.clone() * j).try_inverse()? * j_t;

            solution -= j_i * self.run(&arg_array);

            if let Some(sol) = self.verify(&solution) {
                return Some(sol);
            }
        }
        None
    }
    pub fn verify(&self, solution: &DVector<Complex64>) -> Option<Vec<Complex64>> {
        let arg_array: Vec<Complex64> = solution.iter().copied().collect();
        if self.run(&arg_array).iter().all(|v| v.norm() < 0.00000001) {
            Some(arg_array)
        } else {
            None
        }
    }
}
