use crate::parser::{BinaryOpKind, Expr, Expr::BinaryOp};
use crate::simplify::simplify;
use crate::substitute::substitute;

/// Führt eine numerische Approximation des bestimmten Integrals durch.
/// Die Methode verwendet eine einfache Rechteckregel mit sehr kleinen dx-Schritten.
///
/// - `expr`: Der mathematische Ausdruck, der integriert werden soll.
/// - `var`: Die Integrationsvariable.
/// - `lower`: Untere Grenze des Integrals.
/// - `upper`: Obere Grenze des Integrals.
///
/// Gibt das approximierte Integral als `Expr::Number` zurück oder einen Fehler, falls die Berechnung fehlschlägt.
pub fn approx_integral(expr: Expr, var: String, lower: f64, upper: f64) -> Result<Expr, String> {
    let mut result = 0.0;  // Summe für das Integral
    let mut x = lower;  // Startpunkt der Integration
    let dx = 0.0001;  // Schrittweite
    while x < upper {
        let y = simplify(substitute(expr.clone(), var.clone(), Expr::Number(x)), false);
        // Falls die Vereinfachung zu einer Zahl führt, addiere den Rechteckanteil zur Summe
        if let Expr::Number(value) = y {
            result += value * dx;
        } else {
            return Err("Integration failed: expression did not simplify to a number".to_string());
        }
        x += dx;
    }
    Ok(Expr::Number(result))
}

/// Integriert einen polynomartigen Ausdruck symbolisch.
/// Unterstützt Konstanten, Variablen und einfache Summen/Produkte.
///
/// - `expr`: Der Ausdruck, der integriert werden soll.
/// - `var`: Die Variable, nach der integriert wird.
///
/// Gibt das integrierte `Expr` zurück oder einen Fehler bei nicht unterstützten Termen.
pub fn integrate_polynomial(expr: Expr, var: String) -> Result<Expr, String> {
    match expr {
        // Eine Konstante wird zur Linearfunktion: ∫a dx = a * x
        Expr::Number(a) => Ok(Expr::BinaryOp(BinaryOpKind::Mul, Box::new(Expr::Number(a)), Box::new(Expr::Var(var)))),

        // Eine Variable wird zu einem Quadratterm: ∫x dx = (1/2) * x^2
        Expr::Var(v) => {
            if v == var {
                Ok(BinaryOp(BinaryOpKind::Mul,
                    Box::new(Expr::Number(0.5)),
                    Box::new(BinaryOp(BinaryOpKind::Pow, Box::new(Expr::Var(v)), Box::new(Expr::Number(2.0))))
                ))
            } else {
                // Falls die Variable nicht übereinstimmt, bleibt sie als Konstante bestehen.
                Ok(BinaryOp(BinaryOpKind::Mul, Box::new(Expr::Var(v)), Box::new(Expr::Var(var))))
            }
        }
        // Falls es sich um eine binäre Operation handelt (Addition, Multiplikation etc.), delegiere an `integrate_binary_op`
        Expr::BinaryOp(op, left, right) => integrate_binary_op(op, *left, *right, var.clone()),
        _ => Err(format!("Error: Unsupported expression in polynomial integration 1: {:?}", expr)),
    }

}

fn integrate_binary_op(op: BinaryOpKind, left: Expr, right: Expr, var: String) -> Result<Expr, String> {
    match (op, left.clone(), right.clone()) {
        // Potenzregel mit Vorfaktor: ∫ a * x^b dx = (a / (b+1)) * x^(b+1)
        (BinaryOpKind::Mul, Expr::Number(a), Expr::BinaryOp(BinaryOpKind::Pow, base, exponent)) => {
            // Dereferencing Box to access inner values
            if let (Expr::Var(x), Expr::Number(b)) = (*base, *exponent) {
                Ok(Expr::BinaryOp(
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
                ))
            } else {
                Err(format!("Error: Unsupported expression in polynomial integration 2: {:?} {:?}", left, right))
            }
        },
        // Integral der Summe ist Summe der Integrale
        (BinaryOpKind::Add, left, right) => {
            let left_integral = integrate_polynomial(left.clone(), var.clone());
            let right_integral = integrate_polynomial(right.clone(), var.clone());

            match (left_integral, right_integral) {
                (Ok(left_expr), Ok(right_expr)) => {
                    Ok(Expr::BinaryOp(
                        BinaryOpKind::Add,
                        Box::new(left_expr),
                        Box::new(right_expr),
                    ))
                }
                _ => Err(format!("Error: Unsupported expression in polynomial integration 3: {:?} {:?}", left, right)),
            }
        }
        // Einfache Multiplikation mit Konstante: ∫a*x dx = (a/2) * x^2
        (BinaryOpKind::Mul, Expr::Number(a), Expr::Var(v)) => {
            if v == var {
                Ok(BinaryOp(BinaryOpKind::Mul,
                    Box::new(Expr::Number(a * 0.5)),
                    Box::new(BinaryOp(BinaryOpKind::Pow, Box::new(Expr::Var(v)), Box::new(Expr::Number(2.0))))
                ))
            } else {
                Ok(BinaryOp(BinaryOpKind::Mul, Box::new(BinaryOp(BinaryOpKind::Mul, Box::new(Expr::Number(a)), Box::new(Expr::Var(v)))), Box::new(Expr::Var(var))))
            }
        }
        // Potenzregel ohne Vorfaktor: ∫x^n dx = (1 / (n+1)) * x^(n+1)
        (BinaryOpKind::Pow, Expr::Var(v), Expr::Number(exponent)) => {
            if v == var {
                Ok(BinaryOp(BinaryOpKind::Mul,
                    Box::new(Expr::Number(1.0 / (exponent + 1.0))),
                    Box::new(BinaryOp(BinaryOpKind::Pow, Box::new(Expr::Var(v)), Box::new(Expr::Number(exponent + 1.0))))
                ))
            } else {
                Err(format!("Error: Unsupported expression in polynomial integration 4: {:?} {:?} {:?}", op, left, right))
            }
        }
        _ => Err(format!("Error: Unsupported expression in polynomial integration 4: {:?} {:?} {:?}", op, left, right)),
    }
}
