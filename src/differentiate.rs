use crate::parser::{BinaryOpKind, Expr};
use crate::parser::Expr::Number;

// Die Hauptfunktion zur Ableitung eines Ausdrucks nach einer Variablen.
pub fn differentiate(expr: Expr, var: String) -> Expr {
    match expr {
        Expr::Number(_) => Expr::Number(0.0), // Konstante Zahlen haben immer die Ableitung 0

        // Die Ableitung einer Variablen ist 1, wenn sie mit der gesuchten Variablen übereinstimmt,
        // andernfalls ist sie 0 (da sie dann als Konstante betrachtet wird)
        Expr::Var(v) => {
            if v == var {
                Expr::Number(1.0)
            } else {
                Expr::Number(0.0)
            }
        }
        // Differentiation für binäre Operationen wie +, -, *, /
        Expr::BinaryOp(op, left, right) => diff_binary_op(op, *left, *right, var),

        // Differentiation für unäre Operationen (wie Negation)
        Expr::UnaryOp(op, expr) => diff_unary_op(op, *expr, var),

        // Differentiation für Funktionsaufrufe
        Expr::Call(expr, args) => diff_function(*expr, args, var),
        _ => panic!("Not implemented"),
    }
}

fn diff_binary_op(op: crate::parser::BinaryOpKind, left: Expr, right: Expr, var: String) -> Expr {
    match op {
        // Ableitung von Addition und Subtraktion erfolgt komponentenweise
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

        // Ableitung von Multiplikation mit Produktregel
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

        // Ableitung von Division mit Quotientenregel
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

        // Ableitung von Potenzfunktionen
        crate::parser::BinaryOpKind::Pow => {
            // Ableitung von f(x)^g(x) mit der Regel:
            // General case: (f(x)^g(x))' = g(x) f(x)^(g(x)-1) f'(x) + f(x)^g(x) ln(f(x)) g'(x)
            let base = left.clone();
            let exponent = right.clone();
            let base_diff = differentiate(base.clone(), var.clone());
            let exponent_diff = differentiate(exponent.clone(), var.clone());

            let term1 = Expr::BinaryOp(
                crate::parser::BinaryOpKind::Mul,
                Box::new(exponent.clone()), // g(x)
                Box::new(Expr::BinaryOp(
                    crate::parser::BinaryOpKind::Mul,
                    Box::new(Expr::BinaryOp(
                        crate::parser::BinaryOpKind::Pow,
                        Box::new(base.clone()),
                        Box::new(Expr::BinaryOp(
                            crate::parser::BinaryOpKind::Sub,
                            Box::new(exponent.clone()),
                            Box::new(Expr::Number(1.0)),
                        )),
                    )),
                    Box::new(base_diff),
                )),
            );

            let term2 = Expr::BinaryOp(
                crate::parser::BinaryOpKind::Mul,
                Box::new(Expr::BinaryOp(
                    crate::parser::BinaryOpKind::Mul,
                    Box::new(Expr::BinaryOp(
                        crate::parser::BinaryOpKind::Pow,
                        Box::new(base.clone()),
                        Box::new(exponent.clone()),
                    )),
                    Box::new(Expr::BinaryOp(
                        crate::parser::BinaryOpKind::Mul,
                        Box::new(Expr::Var("ln".to_string())),
                        Box::new(base.clone()),
                    )),
                )),
                Box::new(exponent_diff),
            );

            Expr::BinaryOp(crate::parser::BinaryOpKind::Add, Box::new(term1), Box::new(term2))
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

// Prüft, ob eine Funktion elementar ist (z.B. exp, ln, sin, cos)
pub fn is_elementary_function(name: &str) -> bool {
    name == "exp" || name == "ln" ||  name == "sin" || name == "cos" || name == "id"
}

// Ableitungen von elementaren Funktionen
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
        "sin" => Expr::Var("cos".to_string()),
        "cos" => Expr::UnaryOp(crate::parser::UnaryOpKind::Neg, Box::new(Expr::Var("sin".to_string()))),
        _ => panic!("Not implemented"),
    }
}