use crate::differentiate::differentiate;
use crate::plot::{plot, values_table};
use crate::render::render_latex;
use crate::scanner::Scanner;
use crate::simplify::simplify;
//
mod scanner;
mod parser;
mod differentiate;
mod eval;
mod render;
mod simplify;
mod constants;
mod plot;

fn main() {
    // let source = "a * (b + c)";
    // let source = "5 * (x^2) + 2 * x + 7";
    let source = "7 * (x^4) - 3 * (x^3) + 5 * (x^2) - 8 * x + 2";
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


    println!("expression: {:?}", expression);
    println!("expression: {:?}", simplify(expression.clone(), false));

    plot(expression.clone());

    // let (x_values, y_values) = values_table(expression);
    //
    // // Print x-values and corresponding y-values
    // for (x, y) in x_values.iter().zip(y_values.iter()) {
    //     println!("{:?}: {:?}", x, y);
    // }
}



