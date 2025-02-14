use crate::differentiate::differentiate;
use crate::render::{render_text, render_latex};
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

#[tokio::main]
async fn main() {
    let source = "2 * x";
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

    /*
    println!("{:?}", expression);
    println!("{:?}", differentiate(expression.clone(), "x".to_string()));
    println!("{}", render_latex(differentiate(expression.clone(), "x".to_string())));
    println!("{}", render_latex(simplify(expression)));
    */

    let int = integrate(expression, "x".to_string(), 0.0, 3.0);

    if let Ok(Expr::Number(value)) = int {
        println!("{}", value);
    } else {
        println!("Error: {}", int.unwrap_err());
    }
}
