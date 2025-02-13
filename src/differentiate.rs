use crate::parser::Expr;
use crate::parser::Expr::Number;

pub fn differentiate(expr: Expr, var: String) -> Expr {
    match expr {
        Expr::Number(_) => Expr::Number(0.0),
        Expr::Var(v) => {
            if v == var {
                Expr::Number(1.0)
            } else {
                Expr::Number(0.0)
            }
        }
        Expr::BinaryOp(op, left, right) => diff_binary_op(op, *left, *right, var),
        Expr::UnaryOp(op, expr) => diff_unary_op(op, *expr, var),
        Expr::Call(expr, args) => diff_function(*expr, args, var),
        _ => panic!("Not implemented"),
    }
}

fn diff_binary_op(op: crate::parser::BinaryOpKind, left: Expr, right: Expr, var: String) -> Expr {
    match op {
        crate::parser::BinaryOpKind::Add => Expr::BinaryOp(
            crate::parser::BinaryOpKind::Add,
            Box::new(differentiate(left, var.clone())),
            Box::new(differentiate(right, var)),
        ),
        crate::parser::BinaryOpKind::Sub => Expr::BinaryOp(
            crate::parser::BinaryOpKind::Sub,
            Box::new(differentiate(left, var.clone())),
            Box::new(differentiate(right, var)),
        ),
        crate::parser::BinaryOpKind::Mul => Expr::BinaryOp(
            crate::parser::BinaryOpKind::Add,
            Box::new(Expr::BinaryOp(
                crate::parser::BinaryOpKind::Mul,
                Box::new(differentiate(left.clone(), var.clone())),
                Box::new(right.clone()),
            )),
            Box::new(Expr::BinaryOp(
                crate::parser::BinaryOpKind::Mul,
                Box::new(left.clone()),
                Box::new(differentiate(right, var)),
            )),
        ),
        crate::parser::BinaryOpKind::Div => {
            let left_diff = differentiate(left.clone(), var.clone());
            let right_diff = differentiate(right.clone(), var.clone());
            let left_mul_right = Expr::BinaryOp(
                crate::parser::BinaryOpKind::Mul,
                Box::new(left_diff),
                Box::new(right.clone()),
            );
            let right_mul_left = Expr::BinaryOp(
                crate::parser::BinaryOpKind::Mul,
                Box::new(left.clone()),
                Box::new(right_diff),
            );
            let numerator = Expr::BinaryOp(
                crate::parser::BinaryOpKind::Sub,
                Box::new(left_mul_right),
                Box::new(right_mul_left),
            );
            let denominator = Expr::BinaryOp(
                crate::parser::BinaryOpKind::Mul,
                Box::new(right.clone()),
                Box::new(right.clone()),
            );
            Expr::BinaryOp(crate::parser::BinaryOpKind::Div, Box::new(numerator), Box::new(denominator))
        },
        crate::parser::BinaryOpKind::Pot => {
            let left = left.clone();
            let right = right.clone();
            let left_diff = differentiate(left.clone(), var.clone());
            let right_diff = differentiate(right.clone(), var.clone());
            let left_pow_right = Expr::BinaryOp(
                crate::parser::BinaryOpKind::Pot,
                Box::new(left.clone()),
                Box::new(right.clone()),
            );
            let left_mul_right_pow_left_sub_one = Expr::BinaryOp(
                crate::parser::BinaryOpKind::Mul,
                Box::new(left_diff),
                Box::new(Expr::BinaryOp(
                    crate::parser::BinaryOpKind::Pot,
                    Box::new(left.clone()),
                    Box::new(Expr::BinaryOp(
                        crate::parser::BinaryOpKind::Sub,
                        Box::new(right.clone()),
                        Box::new(Number(1.0)),
                    )),
                )),
            );
            let right_mul_left_pow_right = Expr::BinaryOp(
                crate::parser::BinaryOpKind::Mul,
                Box::new(right_diff),
                Box::new(left_pow_right),
            );
            Expr::BinaryOp(
                crate::parser::BinaryOpKind::Add,
                Box::new(left_mul_right_pow_left_sub_one),
                Box::new(right_mul_left_pow_right),
            )
        }
    }
}

fn diff_unary_op(op: crate::parser::UnaryOpKind, expr: Expr, var: String) -> Expr {
    match op {
        crate::parser::UnaryOpKind::Neg => Expr::UnaryOp(crate::parser::UnaryOpKind::Neg, Box::new(differentiate(expr, var))),
    }
}

fn diff_function(expr: Expr, args: Vec<Expr>, var: String) -> Expr {
    match expr.clone() {
        Expr::Var(v) => {
            let mut result = Number(0.0);
            for i in 0..args.len() {
                let arg = args[i].clone();
                let mut f_prime;
                if is_elementary_function(&v) {
                    f_prime = differentiate_elementary_function(&v, i, var.clone());
                } else {
                    f_prime = differentiate(expr.clone(), var.clone());
                }
                f_prime = Expr::Call(Box::new(f_prime), args.clone());
                result = Expr::BinaryOp(
                    crate::parser::BinaryOpKind::Add,
                    Box::new(result),
                    Box::new(Expr::BinaryOp(
                        crate::parser::BinaryOpKind::Mul,
                        Box::new(f_prime.clone()),
                        Box::new(differentiate(arg, var.clone())),
                    )),
                );
            }
            result
        }
        _ => panic!("Not implemented"),
    }
}

pub fn is_elementary_function(name: &str) -> bool {
    name == "exp" || name == "ln" || name == "id"
}

fn differentiate_elementary_function(name: &str, arg_index: usize, var: String) -> Expr {
    match name {
        "exp" => Expr::Var("exp".to_string()),
        "ln" => {
            if arg_index == 0 {
                Expr::BinaryOp(
                    crate::parser::BinaryOpKind::Div,
                    Box::new(Number(1.0)),
                    Box::new(Expr::Var("id".to_string())),
                )
            } else {
                panic!("log function can only have one argument");
            }
        },
        _ => panic!("Not implemented"),
    }
}