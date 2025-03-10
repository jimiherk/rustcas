use crate::differentiate::differentiate;
use crate::render::{render_latex};
use crate::scanner::Scanner;
use crate::simplify::simplify;
use crate::integrate::integrate;
use crate::parser::Expr;

mod scanner;
mod parser;
mod differentiate;
mod eval;
mod render;
mod simplify;
mod integrate;
mod substitute;
mod constants;
mod plot;

fn main() {
    // let source = "a * (b + c)";
    // let source = "(x^3) + 3 * (x^2) + 2";
    let source = "cos(x)";
    // let source = "7 * (x^4) - 3 * (x^3) + 5 * (x^2) - 8 * x + 2";
    let mut scanner = Scanner::new(source);
    let mut tokens = vec![];
    while let token = scanner.scan_token() {
        tokens.push(token);
        if token.kind == scanner::TokenType::Eof {
            break;
        }
    }
    let mut parser = parser::Parser::new(tokens);
    let expression = parser.expression();

    println!("expression: {:?}", expression.clone());
    println!("expression: {:?}", simplify(expression.clone(), false));
    println!("{}", render_latex(simplify(differentiate(expression.clone(), "x".to_string()), false)));
}
