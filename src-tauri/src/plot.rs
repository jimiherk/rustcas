use crate::simplify::simplify;
use crate::parser::{BinaryOpKind, Expr};
use std::collections::HashMap;

pub fn substitute_for_variable(expr: Expr, variables: &HashMap<String, Expr>) -> Expr {
    match expr {
        Expr::Var(var) => {
            if let Some(value) = variables.get(&var) {
                value.clone()
            } else {
                panic!("Variable {} not found", var);
            }
        }
        Expr::BinaryOp(op, left, right) => {
            Expr::BinaryOp(
                op,
                Box::new(substitute_for_variable(*left, variables)),
                Box::new(substitute_for_variable(*right, variables)),
            )
        }
        _ => expr,
    }
}