use crate::differentiate::differentiate;
use crate::render::render_latex;
use crate::scanner::Scanner;
use crate::simplify::simplify;

mod scanner;
mod parser;
mod differentiate;
mod eval;
mod render;
mod simplify;

fn main() {
    let source = "exp(ln(x) * x*exp(x))";
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
    println!("{:?}", expression);
    println!("{:?}", differentiate(expression.clone(), "x".to_string()));
    println!("{}", render_latex(differentiate(expression.clone(), "x".to_string())));
    println!("{}", render_latex(simplify(differentiate(expression, "x".to_string()))));

}
