//===========================================================================================================================
// IMPORTS
//===========================================================================================================================

use std::sync::Arc;
use axum::{extract::{Json, State}, http::StatusCode};
use ort::{value::Tensor};
use ndarray::{Array1};

use crate::state::AppState;
use crate::responses::response::{PredictRequest, PredictResponse};

//===========================================================================================================================
// ONNX PREDICTION
//===========================================================================================================================

pub async fn predict(
    State(state): State<Arc<AppState>>,
    Json(request): Json<PredictRequest>
) -> Result<Json<PredictResponse>, StatusCode> {
    let input_array = Array1::from(request.input);

    let input = Tensor::from_array(input_array)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut inference_session = state
        .session
        .write()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let outputs = inference_session
        .run(ort::inputs![input])
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let output = outputs
        .get("output")
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let (_, output_data) = output
        .try_extract_tensor::<f32>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(PredictResponse {
        output: output_data.to_vec(),
    }))
}