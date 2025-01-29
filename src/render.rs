use crate::differentiate::is_elementary_function;
use crate::parser::Expr;

pub fn render_latex(expr: Expr) -> String {
    match expr {
        Expr::Number(n) => n.to_string(),
        Expr::Var(v) =>
            if is_elementary_function(&v) {
                render_elementary_function(&v)
            } else {
                v
            },
        Expr::BinaryOp(op, left, right) => {
            let left = render_latex(*left);
            let right = render_latex(*right);
            match op {
                crate::parser::BinaryOpKind::Add => format!("({} + {})", left, right),
                crate::parser::BinaryOpKind::Sub => format!("({} - {})", left, right),
                crate::parser::BinaryOpKind::Mul => format!("({} \\cdot {})", left, right),
                crate::parser::BinaryOpKind::Div => format!("\\frac{{ {} }}{{ {} }}", left, right),
            }
        }
        Expr::Call(func, args) => {
            let f = render_latex(*func.clone());
            let args = args.iter().map(|arg| render_latex(arg.clone())).collect::<Vec<String>>().join(", ");
            if let Expr::Var(name) = *func {
                if is_elementary_function(&name) {
                    return format!("{}\\left({}\\right)", render_elementary_function(&name), args);
                }
                return format!("{}\\left({}\\right)", name, args);
            }
            format!("\\left[{}\\right]\\left({}\\right)", f, args)
        }
        Expr::UnaryOp(op, expr) => {
            let expr = render_latex(*expr);
            match op {
                crate::parser::UnaryOpKind::Neg => format!("-{}", expr),
            }
        }
    }
}

fn render_elementary_function(name: &str) -> String {
    match name {
        "exp" => "\\exp".to_string(),
        "ln" => "\\ln".to_string(),
        "id" => "\\mathrm{id}".to_string(),
        _ => panic!("Not implemented"),
    }
}
