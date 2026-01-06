use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::error::{Error, Result};

pub fn routes() -> Router {
     Router::new().route("/api/login", post(login))
}

async fn login(login_dto: Json<LoginDto>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    if login_dto.username != "demo1" || login_dto.password != "welcome" {
        return Err(Error::LoginFail)
    }

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    todo!()
}

#[derive(Debug, Deserialize)]
struct LoginDto {
    username: String,
    password: String
}