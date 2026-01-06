#![allow(unused)] // To be deleted later!

use axum::{Router, extract::{Path, Query}, response::{Html, IntoResponse}, routing::{get, get_service}};
use serde::Deserialize;
use tower_http::services::ServeDir;
use std::{net::SocketAddr};

mod error;

#[tokio::main]
async fn main() {
    let route = Router::new().merge(route_fx());

    // region:  -- Start Server
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(route.into_make_service())
        .await 
        .unwrap()
}

/* Route method */
fn route_fx() -> Router { 
    Router::new().route(
        "/hello",
        get(handler_query_fx)
    )
    .route("/hello2/:name", get(handler_path_fx))
    .fallback_service(route_static_fx()) // A fallback service
}

/* Static routing */
fn route_static_fx() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("/")))
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

 

