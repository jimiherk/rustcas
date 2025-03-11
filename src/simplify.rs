use crate::differentiate::is_elementary_function;
use crate::constants::E;
use crate::parser::{BinaryOpKind, Expr};

pub fn simplify(expr: Expr, s: bool) -> Expr {
    // s = simplify_elementary_function
    match expr {
        Expr::Number(_) => expr,
        Expr::Var(_) => expr,
        Expr::BinaryOp(op, left, right) => {
            let left = simplify(*left, s);
            let right = simplify(*right, s);
            match (op, left.clone(), right.clone()) {
                (BinaryOpKind::Add, Expr::Number(a), Expr::Number(b)) => Expr::Number(a + b),
                (BinaryOpKind::Sub, Expr::Number(a), Expr::Number(b)) => Expr::Number(a - b),
                (BinaryOpKind::Mul, Expr::Number(a), Expr::Number(b)) => Expr::Number(a * b),
                (BinaryOpKind::Div, Expr::Number(a), Expr::Number(b)) => Expr::Number(a / b),
                (BinaryOpKind::Pow, Expr::Number(_a), Expr::Number(0.0)) => Expr::Number(1.0),
                (BinaryOpKind::Pow, Expr::Number(a), Expr::Number(1.0)) => Expr::Number(a),
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
            let value = simplify_call(*func, &args, s);
            match value {
                Expr::Call(func, args) => Expr::Call(func, args),
                _ => simplify(value, s),
            }
        }
        Expr::UnaryOp(op, expr) => {
            let expr = simplify(*expr, s);
            match (op, expr.clone()) {
                (crate::parser::UnaryOpKind::Neg, Expr::Number(x)) => Expr::Number(-x),
                (crate::parser::UnaryOpKind::Neg, Expr::BinaryOp(BinaryOpKind::Add, left, right)) => {
                    Expr::BinaryOp(
                        BinaryOpKind::Add,
                        Box::new(simplify(Expr::UnaryOp(crate::parser::UnaryOpKind::Neg, left), s)),
                        Box::new(simplify(Expr::UnaryOp(crate::parser::UnaryOpKind::Neg, right), s)),
                    )
                }
                (crate::parser::UnaryOpKind::Neg, Expr::BinaryOp(BinaryOpKind::Sub, left, right)) => {
                    Expr::BinaryOp(
                        BinaryOpKind::Sub,
                        Box::new(simplify(Expr::UnaryOp(crate::parser::UnaryOpKind::Neg, left), s)),
                        right,
                    )
                }

                _ => Expr::UnaryOp(op, Box::new(expr)),
            }
        }
    }
}

fn simplify_call(func: Expr, args: &Vec<Expr>, s: bool) -> Expr {
    // s = simplify_elementary_function
    let args: Vec<Expr> = args.iter().map(|arg| simplify(arg.clone(), s)).collect();
    match func.clone() {
        Expr::Var(name) => {
            if name == "id" {
                assert!(args.len() == 1);
                return args[0].clone()
            } else if is_elementary_function(&name){
                if s {
                    evaluate_elementary_function(&name);
                }
                return Expr::Call(Box::new(func.clone()), args.clone());
            }
            Expr::Var(name.clone())
        },
        Expr::Number(_) => func.clone(),
        Expr::Call(func, args) => {
            simplify_call(*func, &args, s)
        },
        Expr::UnaryOp(op, expr) => {
            Expr::UnaryOp(op, Box::new(simplify_call(*expr, &args, s)))
        },
        Expr::BinaryOp(op, left, right) => {
            Expr::BinaryOp(op, Box::new(simplify_call(*left, &args, s)), Box::new(simplify_call(*right, &args, s)))
        },
        _ => Expr::Call(Box::new(func.clone()), args.clone()),
    }
}

fn evaluate_elementary_function(name: &str) -> Expr {
    // TODO
    match name {
        _ => panic!("Not implemented")
    }
}