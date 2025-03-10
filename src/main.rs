use wasm_bindgen::prelude::wasm_bindgen;
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

fn main() {
    let source = "2 * x * 3";
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
}

#[wasm_bindgen]
pub fn calc(input: String) -> String {
    let mut scanner = Scanner::new(&input);
    let mut tokens = vec![];
    while let token = scanner.scan_token() {
        tokens.push(token);
        if token.kind == scanner::TokenType::Eof {
            break;
        }
    }
    let mut parser = parser::Parser::new(tokens);
    let expression = parser.expression();
    let simplified = simplify(expression.clone(), false);
    let simplified_s = simplify(expression, true);
    let differentiated = differentiate(simplified.clone(), "x".to_string());
    let latex = render_latex(simplified);
    let latex_diff = render_latex(differentiated);

    let result = format!("{{simplified: {}, differentiated: {}}}", latex, latex_diff);

    return result;
}