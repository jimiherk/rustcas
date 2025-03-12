use crate::differentiate::is_elementary_function;
use crate::parser::{BinaryOpKind, Expr, UnaryOpKind};

/// Hauptfunktion, die einen Ausdruck in LaTeX rendert.
pub fn render_latex(expr: &Expr) -> String {
    match expr {
        Expr::Number(n) => n.to_string(),
        Expr::Var(v) => {
            if is_elementary_function(v) {
                render_elementary_function(v)
            } else {
                v.clone()
            }
        }
        // Addition und Subtraktion werden gemeinsam behandelt
        Expr::BinaryOp(op, _, _) if matches!(op, BinaryOpKind::Add | BinaryOpKind::Sub) => {
            render_add(expr)
        }
        // Multiplikation und Division werden gemeinsam behandelt
        Expr::BinaryOp(op, _, _) if matches!(op, BinaryOpKind::Mul | BinaryOpKind::Div) => {
            render_mul(expr)
        }
        // Exponentiation wird direkt behandelt
        Expr::BinaryOp(BinaryOpKind::Pow, left, right) => {
            render_pow(left, right)
        }
        Expr::Call(func, args) => {
            let f = render_latex(func);
            let args_tex = args.iter().map(|arg| render_latex(arg)).collect::<Vec<_>>().join(", ");
            if let Expr::Var(name) = &**func {
                if is_elementary_function(name) {
                    return format!("{}\\left({}\\right)", render_elementary_function(name), args_tex);
                }
                return format!("{}\\left({}\\right)", name, args_tex);
            }
            format!("\\left[{}\\right]\\left({}\\right)", f, args_tex)
        }
        Expr::UnaryOp(op, e) => {
            let inner = render_latex(e);
            match op {
                UnaryOpKind::Neg => format!("-{}", inner),
            }
        }
        _ => unimplemented!(),
    }
}

/// Erzeugt eine Liste von Additions-/Subtraktionstermen aus einem verschachtelten binären Add-/Sub-Ausdruck.
/// Jeder Term wird als Tupel (negiert, &Expr) zurückgegeben – dabei signalisiert das bool,
/// ob der Term negativ sein soll.
fn flatten_add_sub(expr: &Expr) -> Vec<(bool, &Expr)> {
    match expr {
        // Bei Addition: Beide Seiten normal weiterflachen.
        Expr::BinaryOp(BinaryOpKind::Add, left, right) => {
            let mut terms = flatten_add_sub(left);
            terms.extend(flatten_add_sub(right));
            terms
        }
        // Bei Subtraktion: Die linke Seite normal, die rechte Seite als negierter Term.
        Expr::BinaryOp(BinaryOpKind::Sub, left, right) => {
            let mut terms = flatten_add_sub(left);
            // Bei der rechten Seite werden alle Vorzeichen umgekehrt.
            let right_terms = flatten_add_sub(right)
                .into_iter()
                .map(|(neg, term)| (!neg, term))
                .collect::<Vec<_>>();
            terms.extend(right_terms);
            terms
        }
        _ => vec![(false, expr)],
    }
}

/// Rendert einen Addition-/Subtraktionsausdruck, indem alle Terme aus der Flatten‑Funktion zusammengefügt werden.
fn render_add(expr: &Expr) -> String {
    let terms = flatten_add_sub(expr);
    let mut result = String::new();
    for (i, (neg, term)) in terms.iter().enumerate() {
        let term_str = render_latex(term);
        if i == 0 {
            if *neg {
                result.push_str("-");
            }
            result.push_str(&term_str);
        } else {
            if *neg {
                result.push_str("-");
            } else {
                result.push_str("+");
            }
            result.push_str(&term_str);
        }
    }
    result
}

/// Erzeugt eine Liste von Faktoren aus einem verschachtelten Multiplikations-/Divisionsausdruck.
/// Jeder Faktor wird als Tupel (invertiert, &Expr) zurückgegeben – wobei „invertiert“ signalisiert, dass der Faktor
/// im Nenner stehen soll.
fn flatten_mul_div(expr: &Expr) -> Vec<(bool, &Expr)> {
    match expr {
        Expr::BinaryOp(BinaryOpKind::Mul, left, right) => {
            let mut factors = flatten_mul_div(left);
            factors.extend(flatten_mul_div(right));
            factors
        }
        Expr::BinaryOp(BinaryOpKind::Div, left, right) => {
            let mut factors = flatten_mul_div(left);
            // Bei Division: Die rechten Faktoren werden invertiert.
            let right_factors = flatten_mul_div(right)
                .into_iter()
                .map(|(inv, term)| (!inv, term))
                .collect::<Vec<_>>();
            factors.extend(right_factors);
            factors
        }
        _ => vec![(false, expr)],
    }
}

/// Rendert einen Multiplikations-/Divisionsausdruck.
fn render_mul(expr: &Expr) -> String {
    let factors = flatten_mul_div(expr);
    let mut numerators = Vec::new();
    let mut denominators = Vec::new();
    for (inv, factor) in factors {
        if inv {
            denominators.push(render_latex(factor));
        } else {
            numerators.push(render_latex(factor));
        }
    }
    let num_str = if numerators.is_empty() { "1".to_string() } else { numerators.join(" \\cdot ") };
    if denominators.is_empty() {
        num_str
    } else {
        let den_str = denominators.join(" \\cdot ");
        format!("\\frac{{{}}}{{{}}}", num_str, den_str)
    }
}

/// Rendert einen Potenzausdruck. Sonderfälle:
/// - x⁻¹ wird als Bruch dargestellt.
/// - x^(1/2) wird als Quadratwurzel dargestellt.
/// - Bei trigonometrischen Funktionen im Basis-Ausdruck wird der Exponent als Superscript am Funktionsnamen angebracht.
fn render_pow(base: &Expr, exp: &Expr) -> String {
    // Sonderfall: x^(-1) als Bruch
    if let Expr::Number(n) = exp {
        if *n == -1.0 {
            return format!("\\frac{{1}}{{{}}}", render_latex(base));
        }
        // Sonderfall: Quadratwurzel, falls 1/2 als Exponent gegeben
        if *n == 0.5 {
            return format!("\\sqrt{{{}}}", render_latex(base));
        }
    }
    // Sonderfall: Trigonometrische Funktionen, z. B. sin(x)^2 → \sin^{2}(x)
    if let Expr::Call(func, args) = base {
        if let Expr::Var(ref name) = **func {
            if name == "sin" || name == "cos" {
                let arg_tex = if let Some(arg) = args.get(0) {
                    render_latex(arg)
                } else {
                    "".to_string()
                };
                let fun_tex = render_elementary_function(name);
                return format!("{}^{{{}}}\\left({}\\right)", fun_tex, render_latex(exp), arg_tex);
            }
        }
    }
    // Standardfall: Falls der Basis-Ausdruck zusammengesetzt ist, werden Klammern gesetzt.
    let base_tex = match base {
        Expr::BinaryOp(_, _, _) | Expr::Call(_, _) | Expr::UnaryOp(_, _) => format!("({})", render_latex(base)),
        _ => render_latex(base),
    };
    format!("{}^{{{}}}", base_tex, render_latex(exp))
}

/// Rendert elementare Funktionen in LaTeX.
fn render_elementary_function(name: &str) -> String {
    match name {
        "exp" => "\\exp".to_string(),
        "sin" => "\\sin".to_string(),
        "cos" => "\\cos".to_string(),
        "ln"  => "\\ln".to_string(),
        "id"  => "\\mathrm{id}".to_string(),
        _ => panic!("Nicht implementiert"),
    }
}