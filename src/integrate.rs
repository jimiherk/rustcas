extern crate dotenv;

use crate::parser::Expr;
use crate::simplify::simplify;
use crate::substitute::substitute;

pub fn integrate(expr: Expr, var: String, lower: f64, upper: f64) -> Result<Expr, String> {
    let mut result = 0.0;
    let mut x = lower;
    let dx = 0.0001;
    while x < upper {
        let y = simplify(substitute(expr.clone(), var.clone(), Expr::Number(x)));
        if let Expr::Number(value) = y {
            result += value * dx;
        } else {
            return Err("Integration failed: expression did not simplify to a number".to_string());
        }
        x += dx;
    }
    Ok(Expr::Number(result))
}