use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

use crate::{error::{Error, Result}, web::AUTH_TOKEN};

pub fn routes() -> Router {
     Router::new().route("/api/login", post(login))
}

async fn login(cookies: Cookies, login_dto: Json<LoginDto>) -> Result<Json<Value>> {
    debug!("->> {:<12} - api_login", "HANDLER");

    if login_dto.username != "demo1" || login_dto.password != "welcome" {
        return Err(Error::LoginFail)
    }

    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginDto {
    username: String,
    password: String
}