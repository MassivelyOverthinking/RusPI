//===========================================================================================================================
// IMPORTS
//===========================================================================================================================

use std::sync::Arc;
use axum::Router;
use ort::ErrorCode::Ok;

use crate::routes::inference::inference_routes;
use crate::state::AppState;

//===========================================================================================================================
// MODULES
//===========================================================================================================================

mod onnx;
mod routes;
mod controllers;
mod models;
mod cache;
mod state;
mod error;
mod configuration;

//===========================================================================================================================
// MAIN
//===========================================================================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(AppState::new()?);

    let app = Router::new()
        .merge(inference_routes())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server listening on {}", listener.local_adr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
