use crate::parser::Expr;

pub fn substitute (expr: Expr, var: String, value: Expr) -> Expr {
    match expr {
        Expr::Number(_) => expr,
        Expr::Var(v) => {
            if v == var {
                value
            } else {
                Expr::Var(v)
            }
        }
        Expr::BinaryOp(op, left, right) => Expr::BinaryOp(
            op,
            Box::new(substitute(*left, var.clone(), value.clone())),
            Box::new(substitute(*right, var, value)),
        ),
        Expr::UnaryOp(op, expr) => Expr::UnaryOp(op, Box::new(substitute(*expr, var, value))),
        Expr::Call(expr, args) => Expr::Call(expr, args.into_iter().map(|arg| substitute(arg, var.clone(), value.clone())).collect()),
        _ => panic!("Not implemented"),
    }
}