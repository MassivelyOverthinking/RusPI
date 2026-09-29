//===========================================================================================================================
// IMPORTS
//===========================================================================================================================

use std::sync::Arc;
use axum::Router;

use crate::routes::{inference::inference_routes, health::health_routes};
use crate::state::AppState;

//===========================================================================================================================
// MODULES
//===========================================================================================================================

mod routes;
mod controllers;
mod models;
mod responses;
mod cache;
mod state;

//===========================================================================================================================
// MAIN
//===========================================================================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(AppState::new()?);

    let app = Router::new()
        .merge(inference_routes())
        .merge(health_routes())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    std::result::Result::Ok(())
}
