#![allow(unused)] // To be deleted later!
 
use axum::{Router, extract::{Path, Query}, middleware, response::{Html, IntoResponse, Response}, routing::{get, get_service}};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use std::{net::SocketAddr};

use crate::model::ModelController;

pub use self::error::{Error, Result};

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes_all = Router::new()
        .merge(routes_public())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis(mc.clone()))
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

    println!();
    
    res
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

 

