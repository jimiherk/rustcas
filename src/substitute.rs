use crate::parser::Expr;

pub fn substitute (expr: Expr, var: String, value: Expr) -> Expr {
    match expr {
        // Wenn die Expression eine Zahl ist, wird sie unverändert zurückgegeben
        Expr::Number(_) => expr,
        // Wenn die Expression eine Variable ist, wird sie durch den Wert ersetzt, falls sie mit der gesuchten Variable übereinstimmt
        Expr::Var(v) => {
            if v == var {
                value
            } else {
                Expr::Var(v)
            }
        }
        // Wenn die Expression eine binäre Operation ist, wird die Substitution rekursiv auf die Operanden angewendet
        Expr::BinaryOp(op, left, right) => Expr::BinaryOp(
            op,
            Box::new(substitute(*left, var.clone(), value.clone())),
            Box::new(substitute(*right, var, value)),
        ),
        // Wenn die Expression eine unäre Operation ist, wird die Substitution rekursiv auf die innere Expression angewendet
        Expr::UnaryOp(op, expr) => Expr::UnaryOp(op, Box::new(substitute(*expr, var, value))),
        // Wenn die Expression ein Funktionsaufruf ist, wird die Substitution rekursiv auf die Argumente angewendet
        Expr::Call(expr, args) => Expr::Call(expr, args.into_iter().map(|arg| substitute(arg, var.clone(), value.clone())).collect()),
        // Für alle anderen Fälle wird ein Panic ausgelöst
        _ => panic!("Not implemented"),
    }
}