use std::collections::HashMap;
use crate::differentiate::differentiate;
use crate::integrate::integrate;
use crate::parser::Expr;
use crate::plot::substitute_for_variable;
use crate::render::render_latex;
use crate::scanner::Scanner;
use crate::simplify::simplify;

mod scanner;
mod parser;
mod differentiate;
mod eval;
mod render;
mod simplify;
mod integrate;
mod evaluate;
mod constants;
mod plot;

fn main() {
    // let source = "a * (b + c)";
    // let source = "5 * (x^2) + 2 * x + 7";
    let source = "y * (x + 5)";
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

    let mut variables = HashMap::new();
    variables.insert("x".to_string(), Expr::Number(5.0));
    variables.insert("y".to_string(), Expr::Number(3.5));

    println!("expression: {:?}", expression);
    println!("expression: {:?}", substitute_for_variable(expression, &variables));


}
