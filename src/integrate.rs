extern crate dotenv;

use dotenv::dotenv;
use std::env;
use crate::parser::Expr;
use crate::render::render_text;

pub async fn integrate(expr: Expr, var: String, lower: i32, upper: i32) -> Result<String, reqwest::Error> {
    dotenv().ok();
    let url = format!("http://api.wolframalpha.com/v1/result?appid={}&i=integrate+{}+d{}+from+{}+to+{}", env::var("WA_APP_ID").unwrap(), render_text(expr), var, lower, upper);

    let res = reqwest::get(url)
        .await?
        .text()
        .await?;

    Ok(res)
}