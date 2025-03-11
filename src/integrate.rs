// extern crate dotenv;
//
// use crate::parser::{BinaryOpKind, Expr};
// use crate::simplify::simplify;
// use crate::substitute::substitute;
//
// pub fn integrate(expr: Expr, var: String, lower: f64, upper: f64) -> Result<Expr, String> {
//     let mut result = 0.0;
//     let mut x = lower;
//     let dx = 0.0001;
//     while x < upper {
//         let y = simplify(substitute(expr.clone(), var.clone(), Expr::Number(x)), false);
//         if let Expr::Number(value) = y {
//             result += value * dx;
//         } else {
//             return Err("Integration failed: expression did not simplify to a number".to_string());
//         }
//         x += dx;
//     }
//     Ok(Expr::Number(result))
// }

use crate::parser::{Expr, BinaryOpKind};

pub fn integrate_polynomial(expr: Expr, var: String) -> Expr {
    match expr {
        Expr::Number(a) => Expr::BinaryOp(BinaryOpKind::Mul, Box::new(Expr::Number(a)), Box::new(Expr::Var(var))),
        Expr::Var(v) => {
            if v == var {
                Expr::Number(1.0)
            } else {
                Expr::Number(0.0)
            }
        }
        Expr::BinaryOp(op, left, right) => integrate_binary_op(op, *left, *right, var.clone()),
        _ => panic!("Not implemented"),
    }

}

fn integrate_binary_op(op: BinaryOpKind, left: Expr, right: Expr, var: String) -> Expr {
    match (op, left, right) {
        (BinaryOpKind::Mul, Expr::Number(a), Expr::BinaryOp(BinaryOpKind::Pow, base, exponent)) => {
            // Dereferencing Box to access inner values
            if let (Expr::Var(x), Expr::Number(b)) = (*base, *exponent) {
                Expr::BinaryOp(
                    BinaryOpKind::Mul,
                    Box::new(Expr::BinaryOp(
                        BinaryOpKind::Div,
                        Box::new(Expr::Number(a)),
                        Box::new(Expr::Number(b + 1.0)),
                    )),
                    Box::new(Expr::BinaryOp(
                        BinaryOpKind::Pow,
                        Box::new(Expr::Var(x)),
                        Box::new(Expr::Number(b + 1.0)),
                    )),
                )
            } else {
                panic!("Unsupported exponentiation expression in polynomial integration");
            }
        }
        _ => panic!("Not implemented"),
    }
}
