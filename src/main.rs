#![allow(unused)] // To be deleted later!
 
use axum::{Json, Router, extract::{Path, Query}, middleware, response::{Html, IntoResponse, Response}, routing::{get, get_service}};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::{CookieManagerLayer, service};
use tower_http::services::ServeDir;
use uuid::Uuid;
use std::{net::SocketAddr};

use crate::model::ModelController;

pub use self::error::{Error, Result};

mod error;
mod model;
mod web;
mod ctx;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes_all = Router::new()
        .merge(routes_public())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis(mc.clone()))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::auth_middleware::mw_ctx_resolver
        )) 
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new());


    // region:  -- Start Server
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("->> LISTENING on {addr}\n");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();

    Ok(())
}



// ============================================================================
// Routes - Public
// ============================================================================

fn routes_public() -> Router {
    Router::new()
    .route("/hello", axum::routing::get(|| async { "Hello World!" }))
}


// ============================================================================
// Routes - API (Protected)
// ============================================================================

fn routes_apis(mc: ModelController) -> Router {
    web::routes_tickets::ticket_routes(mc)
        .route_layer(middleware::from_fn(web::auth_middleware::mw_require_auth))
}



async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    /* Get the eventual response error */
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    /* If client error, build the new response */
    let error_response = client_status_error
    .as_ref()
    .map(|(status_code, client_error)| {
        let client_error_body = json!({
            "error": {
                "type": client_error.as_ref(),
                "req_uuid": uuid.to_string()
            }
        });

        println!("     ->> client_error_body: {client_error_body}");
    
        /* Build the new response from the client_error_body */
        (*status_code, Json(client_error_body)).into_response() 
    });
    
    /* Build and log the server log line */
    println!("     ->> server log line: {uuid} - Error: {service_error:?}");

    error_response.unwrap_or(res)
    
}

// serialize - convert Rust to JSON
// deserialize - convert JSON to Rust
#[derive(Debug, Deserialize)] 
struct HelloParams {
    name: Option<String>  // optional, not required
}

/* e.g., `/hello?name=Frank` */
async fn handler_query_fx(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_query_fx - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");

    Html(format!("Hello <strong>{name}</strong>"));
}

/* e.g., `/hello2/Frank` */
async fn handler_path_fx(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_path_fx - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"));
}

 

