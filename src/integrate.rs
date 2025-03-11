use crate::parser::Expr;
use crate::simplify::simplify;
use crate::substitute::substitute;

pub fn integrate(expr: Expr, var: String, lower: f64, upper: f64) -> Result<Expr, String> {
    let mut result = 0.0;
    let mut x = lower;
    let dx = (upper - lower).abs() / 1000.0;
    while x < upper {
        // Ausdruck vereinfachen und substituieren
        let y = simplify(substitute(expr.clone(), var.clone(), Expr::Number(x)), false);
        if let Expr::Number(value) = y {
            result += value * dx;
        } else {
            return Err("Integration fehlgeschlagen: Ausdruck wurde nicht zu einer Zahl vereinfacht".to_string());
        }
        x += dx;
    }
    Ok(Expr::Number(result))
}