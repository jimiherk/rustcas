use crate::differentiate::differentiate;
use crate::render::{render_latex};
use crate::scanner::Scanner;
use crate::simplify::simplify;
use crate::integrate::integrate;
use crate::parser::Expr;
use crate::plot::{plot, substitute_for_variable};

use wasm_bindgen::prelude::*;

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

fn main() {}

#[wasm_bindgen]
pub fn differentiate_expression(expression: String, variable: String) -> String {
    let mut scanner = Scanner::new(&expression);
    let mut tokens = vec![];
    while let token = scanner.scan_token() {
        tokens.push(token);
        if token.kind == scanner::TokenType::Eof {
            break;
        }
    }
    let mut parser = parser::Parser::new(tokens);
    let expression = parser.expression();

    let diff = differentiate(expression.clone(), variable);
    render_latex(simplify(diff.clone(), false))
}

#[wasm_bindgen]
pub fn integrate_expression(expression: String, variable: String, lower: f64, upper: f64) -> String {
    let mut scanner = Scanner::new(&expression);
    let mut tokens = vec![];
    while let token = scanner.scan_token() {
        tokens.push(token);
        if token.kind == scanner::TokenType::Eof {
            break;
        }
    }
    let mut parser = parser::Parser::new(tokens);
    let expression = parser.expression();

    if let Ok(integral) = integrate(expression.clone(), variable, lower, upper) {
        return render_latex(simplify(integral.clone(), false));
    }
    "Error".to_string()
}

#[wasm_bindgen]
pub fn simplify_expression(expression: String) -> String {
    let mut scanner = Scanner::new(&expression);
    let mut tokens = vec![];
    while let token = scanner.scan_token() {
        tokens.push(token);
        if token.kind == scanner::TokenType::Eof {
            break;
        }
    }
    let mut parser = parser::Parser::new(tokens);
    let expression = parser.expression();

    render_latex(simplify(expression.clone(), false))
}

#[wasm_bindgen]
pub fn plot_expression(expression: String) -> Vec<u8> {
    let mut scanner = Scanner::new(&expression);
    let mut tokens = vec![];
    while let token = scanner.scan_token() {
        tokens.push(token);
        if token.kind == scanner::TokenType::Eof {
            break;
        }
    }
    let mut parser = parser::Parser::new(tokens);
    let expression = parser.expression();

    plot(expression)
}