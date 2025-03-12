use crate::differentiate::differentiate;
use crate::render::{render_latex};
use crate::scanner::Scanner;
use crate::simplify::simplify;
use crate::integrate::{integrate_polynomial, approx_integral};
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
    // Scanner initialisieren und Token sammeln
    let mut scanner = Scanner::new(&expression);
    let mut tokens = vec![];
    while let token = scanner.scan_token() {
        tokens.push(token);
        if token.kind == scanner::TokenType::Eof {
            break;
        }
    }
    // Parser initialisieren und Ausdruck parsen
    let mut parser = parser::Parser::new(tokens);
    let expression = parser.expression();

    // Ausdruck differenzieren und als LaTeX rendern
    let diff = differentiate(expression.clone(), variable);
    render_latex(simplify(diff.clone(), false))
}

#[wasm_bindgen]
pub fn integrate_expression(expression: String, variable: String, lower: f64, upper: f64) -> String {
    // Scanner initialisieren und Token sammeln
    let mut scanner = Scanner::new(&expression);
    let mut tokens = vec![];
    while let token = scanner.scan_token() {
        tokens.push(token);
        if token.kind == scanner::TokenType::Eof {
            break;
        }
    }
    // Parser initialisieren und Ausdruck parsen
    let mut parser = parser::Parser::new(tokens);
    let expression = parser.expression();

    // Ausdruck integrieren und als LaTeX rendern
    if let Ok(integral) = approx_integral(expression.clone(), variable, lower, upper) {
        return render_latex(simplify(integral.clone(), false));
    }
    "Error".to_string()
}

#[wasm_bindgen]
pub fn find_antiderivative(expression: String, variable: String) -> Result<String, String> {
    // Scanner initialisieren und Token sammeln
    let mut scanner = Scanner::new(&expression);
    let mut tokens = vec![];
    while let token = scanner.scan_token() {
        tokens.push(token);
        if token.kind == scanner::TokenType::Eof {
            break;
        }
    }
    // Parser initialisieren und Ausdruck parsen
    let mut parser = parser::Parser::new(tokens);
    let expression = parser.expression();

    // Ausdruck integrieren und als LaTeX rendern
    let integral = integrate_polynomial(expression.clone(), variable);
    if let Ok(integral) = integral {
        return Ok(render_latex(simplify(integral.clone(), false)));
    }
    Err(integral.err().unwrap())
}

#[wasm_bindgen]
pub fn simplify_expression(expression: String) -> String {
    // Scanner initialisieren und Token sammeln
    let mut scanner = Scanner::new(&expression);
    let mut tokens = vec![];
    while let token = scanner.scan_token() {
        tokens.push(token);
        if token.kind == scanner::TokenType::Eof {
            break;
        }
    }
    // Parser initialisieren und Ausdruck parsen
    let mut parser = parser::Parser::new(tokens);
    let expression = parser.expression();

    // Ausdruck vereinfachen und als LaTeX rendern
    render_latex(simplify(expression.clone(), false))
}

#[wasm_bindgen]
pub fn plot_expression(expression: String) -> Vec<u8> {
    // Scanner initialisieren und Token sammeln
    let mut scanner = Scanner::new(&expression);
    let mut tokens = vec![];
    while let token = scanner.scan_token() {
        tokens.push(token);
        if token.kind == scanner::TokenType::Eof {
            break;
        }
    }
    // Parser initialisieren und Ausdruck parsen
    let mut parser = parser::Parser::new(tokens);
    let expression = parser.expression();

    // Ausdruck plotten
    plot(expression)
}

#[wasm_bindgen]
pub fn render_latex_expression(expression: String) -> String {
    // Scanner initialisieren und Token sammeln
    let mut scanner = Scanner::new(&expression);
    let mut tokens = vec![];
    while let token = scanner.scan_token() {
        tokens.push(token);
        if token.kind == scanner::TokenType::Eof {
            break;
        }
    }
    // Parser initialisieren und Ausdruck parsen
    let mut parser = parser::Parser::new(tokens);
    let expression = parser.expression();

    // Ausdruck als LaTeX rendern
    render_latex(expression)
}