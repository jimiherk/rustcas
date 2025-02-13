use crate::simplify::simplify;
use crate::parser::{BinaryOpKind, Expr};
use std::collections::HashMap;

fn substitute_for_variable(expr: Expr, variables: &HashMap<String, Expr>) -> Expr {
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

pub fn values_table(expr: Expr) -> (Vec<Expr>, Vec<Expr>) {
    // TODO add iteration over the expression to find all the variables
    // for now the function only works for the variable x

    let mut x_values: Vec<Expr> = Vec::new();
    let mut y_values: Vec<Expr> = Vec::new();
    // i am lowkey sorry about this code
    let mut i = -5.0;
    while i <= 5.0 {
        let mut variables = HashMap::new();
        variables.insert("x".to_string(), Expr::Number(i));

        let simplified_expr = simplify(substitute_for_variable(expr.clone(), &variables), false);

        x_values.push(Expr::Number(i));
        y_values.push(simplified_expr);

        i += 0.5;
    }

    (x_values, y_values)
}