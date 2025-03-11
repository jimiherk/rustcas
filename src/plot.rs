use crate::simplify::simplify;
use crate::parser::{BinaryOpKind, Expr};
use std::collections::HashMap;
use std::io::Cursor;
use image::{RgbImage, Rgb, GenericImage, ImageFormat};

// Ersetzt Variablen in einem Ausdruck durch gegebene Werte
pub fn substitute_for_variable(expr: Expr, variables: &HashMap<String, Expr>) -> Expr {
    match expr {
        Expr::Var(var) => {
            if let Some(value) = variables.get(&var) {
                value.clone()
            } else {
                panic!("Variable {} nicht gefunden", var);
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

// Erstellt eine Wertetabelle für einen Ausdruck
pub fn values_table(expr: Expr) -> (Vec<Expr>, Vec<Expr>) {
    // TODO: Iteration über den Ausdruck hinzufügen, um alle Variablen zu finden
    // Derzeit funktioniert die Funktion nur für die Variable x

    let mut x_values: Vec<Expr> = Vec::new();
    let mut y_values: Vec<Expr> = Vec::new();
    // Entschuldigung für diesen Code
    let mut i = -5.0;
    while i <= 5.0 {
        let mut variables = HashMap::new();
        variables.insert("x".to_string(), Expr::Number(i));

        let simplified_expr = simplify(substitute_for_variable(expr.clone(), &variables), false);

        x_values.push(Expr::Number(i));
        y_values.push(simplified_expr);

        i += 0.01;
    }

    (x_values, y_values)
}

// Plottet einen Ausdruck und gibt das Bild als Byte-Array zurück
pub fn plot(expr: Expr) -> Vec<u8> {
    let (x_values, y_values) = values_table(expr);

    let width = 800;
    let height = 800;

    let mut img = RgbImage::new(width, height);

    // Bestimmt dynamisch die minimalen und maximalen y-Werte
    let min_y = y_values.iter()
        .filter_map(|y| if let Expr::Number(v) = y { Some(*v) } else { None })
        .fold(f64::INFINITY, f64::min);
    let max_y = y_values.iter()
        .filter_map(|y| if let Expr::Number(v) = y { Some(*v) } else { None })
        .fold(f64::NEG_INFINITY, f64::max);

    let min_x = -5.0;
    let max_x = 5.0;

    // Neue dynamische Skalierung
    let scale_x = (width - 1) as f64 / (max_x - min_x);
    let scale_y = (height - 1) as f64 / (max_y - min_y);

    let offset_x = -min_x * scale_x;  // Verschiebt x, um in den Bereich zu passen
    let offset_y = -min_y * scale_y;  // Verschiebt y entsprechend

    // Hintergrund
    for x in 0..width {
        for y in 0..height {
            img.put_pixel(x, y, Rgb([255, 255, 255])); // Weiß
        }
    }

    println!("Debugging x_values und y_values:");

    for (x_expr, y_expr) in x_values.iter().zip(y_values.iter()) {
        if let (Expr::Number(x), Expr::Number(y)) = (x_expr, y_expr) {
            println!("x: {:.2}, y: {:.2}", x, y);
        } else {
            println!("Nicht-numerischer Ausdruck gefunden!");
        }
    }

    // Findet den Index für x=0
    let mut y_axis_pixel_x = None;
    for (i, x_expr) in x_values.iter().enumerate() {
        if let Expr::Number(x) = x_expr {
            if x.abs() < 0.05 { // Nahe null
                y_axis_pixel_x = Some(((*x - min_x) * scale_x) as u32);
                break;
            }
        }
    }

    // Zeichnet die Y-Achse an der richtigen x-Position
    if let Some(y_axis_x) = y_axis_pixel_x {
        if y_axis_x < width {
            for y in 0..height {
                img.put_pixel(y_axis_x, y, Rgb([128, 128, 128])); // Graue Y-Achse
            }
        }
    }

    // Plottet Punkte
    for (x_expr, y_expr) in x_values.iter().zip(y_values.iter()) {
        if let (Expr::Number(x), Expr::Number(y)) = (x_expr, y_expr) {
            let pixel_x = ((*x - min_x) * scale_x) as u32;
            let pixel_y = height.saturating_sub(((*y - min_y) * scale_y) as u32);

            if pixel_x < width && pixel_y < height {
                img.put_pixel(pixel_x, pixel_y, Rgb([0, 0, 255])); // Blauer Pixel
            }
        }
    }

    let mut bytes: Vec<u8> = Vec::new();

    img.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png);

    bytes
}
