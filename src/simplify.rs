use crate::differentiate::is_elementary_function;
use crate::parser::{BinaryOpKind, Expr};

pub fn simplify(expr: Expr) -> Expr {
    match expr {
        Expr::Number(_) => expr,
        Expr::Var(_) => expr,
        Expr::BinaryOp(op, left, right) => {
            let left = simplify(*left);
            let right = simplify(*right);
            match (op, left.clone(), right.clone()) {
                (BinaryOpKind::Add, Expr::Number(a), Expr::Number(b)) => Expr::Number(a + b),
                (BinaryOpKind::Sub, Expr::Number(a), Expr::Number(b)) => Expr::Number(a - b),
                (BinaryOpKind::Mul, Expr::Number(a), Expr::Number(b)) => Expr::Number(a * b),
                (BinaryOpKind::Div, Expr::Number(a), Expr::Number(b)) => Expr::Number(a / b),
                (BinaryOpKind::Pow, Expr::Number(a), Expr::Number(b)) => Expr::Number(a.powf(b)),
                (BinaryOpKind::Mul, Expr::Number(0.0), _) => Expr::Number(0.0),
                (BinaryOpKind::Mul, _, Expr::Number(0.0)) => Expr::Number(0.0),
                (BinaryOpKind::Mul, Expr::Number(1.0), right) => right,
                (BinaryOpKind::Mul, left, Expr::Number(1.0)) => left,
                (BinaryOpKind::Div, left, Expr::Number(1.0)) => left,
                (BinaryOpKind::Add, left, Expr::Number(0.0)) => left,
                (BinaryOpKind::Add, Expr::Number(0.0), right) => right,
                _ => Expr::BinaryOp(op, Box::new(left), Box::new(right)),
            }
        }
        Expr::Call(func, args) => {
            let value = simplify_call(*func, &args);
            match value {
                Expr::Call(func, args) => Expr::Call(func, args),
                _ => simplify(value),
            }
        }
        Expr::UnaryOp(op, expr) => {
            let expr = simplify(*expr);
            match (op, expr.clone()) {
                (crate::parser::UnaryOpKind::Neg, Expr::Number(0.0)) => Expr::Number(0.0),
                _ => Expr::UnaryOp(op, Box::new(expr)),
            }
        }
    }
}

fn simplify_call(func: Expr, args: &Vec<Expr>) -> Expr {
    let args: Vec<Expr> = args.iter().map(|arg| simplify(arg.clone())).collect();
    match func.clone() {
        Expr::Var(name) => {
            if name == "id" {
                assert!(args.len() == 1);
                return args[0].clone()
            } else if is_elementary_function(&name){
                return Expr::Call(Box::new(func.clone()), args.clone());
            }
            Expr::Var(name.clone())
        },
        Expr::Number(_) => func.clone(),
        Expr::Call(func, args) => {
            simplify_call(*func, &args)
        },
        Expr::UnaryOp(op, expr) => {
            Expr::UnaryOp(op, Box::new(simplify_call(*expr, &args)))
        },
        Expr::BinaryOp(op, left, right) => {
            Expr::BinaryOp(op, Box::new(simplify_call(*left, &args)), Box::new(simplify_call(*right, &args)))
        },
        _ => Expr::Call(Box::new(func.clone()), args.clone()),
    }
}