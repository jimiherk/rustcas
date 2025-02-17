use crate::simplify::simplify;
use crate::parser::{BinaryOpKind, Expr};
use std::collections::HashMap;
use image::{RgbImage, Rgb, GenericImage};

fn substitute_for_variable(expr: Expr, variables: &HashMap<String, Expr>) -> Expr {
    match expr {
        Expr::Var(var) => {
            if let Some(value) = variables.get(&var) {
                value.clone()
            } else {
                panic!("Variable {} not found", var);
            }
        }
        Expr::BinaryOp(op, left, right) => {
            Expr::BinaryOp(
                op,
                Box::new(substitute_for_variable(*left, variables)),
                Box::new(substitute_for_variable(*right, variables)),
            )
        }
        _ => expr,
    }
}

pub fn values_table(expr: Expr) -> (Vec<Expr>, Vec<Expr>) {
    // TODO add iteration over the expression to find all the variables
    // for now the function only works for the variable x

    let mut x_values: Vec<Expr> = Vec::new();
    let mut y_values: Vec<Expr> = Vec::new();
    // i am lowkey sorry about this code
    let mut i = -5.0;
    while i <= 5.0 {
        let mut variables = HashMap::new();
        variables.insert("x".to_string(), Expr::Number(i));

        let simplified_expr = simplify(substitute_for_variable(expr.clone(), &variables), false);

        x_values.push(Expr::Number(i));
        y_values.push(simplified_expr);

        i += 0.1;
    }

    (x_values, y_values)
}

pub fn plot(expr: Expr) {
    let (x_values, y_values) = values_table(expr);

    let width = 200;  // Image width (adjust as needed)
    let height = 200; // Image height (adjust as needed)

    let mut img = RgbImage::new(width, height);

    let scale_x = width as f64 / 10.0;  // Maps x from [-5,5] to [0,200]
    let scale_y = height as f64 / 10.0; // Maps y from [-5,5] to [0,200]

    let offset_x = width as f64 / 2.0;  // Centers x at 0
    let offset_y = height as f64 / 2.0;

    for x in 0..width {
        for y in 0..height {
            img.put_pixel(x, y, Rgb([255, 255, 255])); // White background
        }
    }

    for x in 0..width {
        for y in 0..height {
            if (x == width / 2 || y == height / 2) {
                img.put_pixel(x, y, Rgb([128, 128, 128])); // White background
            }
        }
    }

    for (x_expr, y_expr) in x_values.iter().zip(y_values.iter()) {
        if let (Expr::Number(x), Expr::Number(y)) = (x_expr, y_expr) {
            let pixel_x = (*x * scale_x + offset_x) as u32;
            let pixel_y = height.saturating_sub((*y * scale_y + offset_y) as u32); // Flip y-axis

            if pixel_x < width && pixel_y < height {
                img.put_pixel(pixel_x, pixel_y, Rgb([0, 0, 255])); // Blue pixel
            }
        }
    }

    img.put_pixel(0, 0, Rgb([0, 0, 255]));
    img.put_pixel(199, 0, Rgb([0, 0, 255]));
    img.put_pixel(0, 199, Rgb([0, 0, 255]));
    img.put_pixel(199, 199, Rgb([0, 0, 255]));

    img.save("graph.png").unwrap();

}