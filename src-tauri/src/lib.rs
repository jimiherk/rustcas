use crate::differentiate::differentiate;
use crate::render::render_latex;
use crate::scanner::Scanner;
use crate::simplify::simplify;
use crate::integrate::integrate;

mod scanner;
mod parser;
mod differentiate;
mod eval;
mod render;
mod simplify;
mod integrate;
mod substitute;
mod constants;

#[tauri::command]
fn calculate(input: String) -> String {
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
    //let simplified_s = simplify(expression, true);
    let differentiated = simplify(differentiate(simplified.clone(), "x".to_string()), false);
    let latex = render_latex(simplified);
    let latex_diff = render_latex(differentiated);

    let result = format!("{{simplified: {}, differentiated: {}}}", latex, latex_diff);

    return result;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![calculate])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}