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

    //===========================================================================================================================
    // CACHE LOOKUP
    //===========================================================================================================================

    {
        let cache = state
            .cache
            .read()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if let Some(output) = cache.get(&request.input) {
            return Ok(Json(PredictResponse {
                output: output.clone(),
            }));
        }
    }

    //===========================================================================================================================
    // CREATE ONNX INPUT
    //===========================================================================================================================

    let input_array = Array1::from(request.input.clone());

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

    let final_output = output_data.to_vec();

    //===========================================================================================================================
    // ONNX INFERENCE SESSION
    //===========================================================================================================================

    {
        let mut cache = state
            .cache
            .write()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        cache.add(&request.input, final_output.clone());
    }

    //===========================================================================================================================
    // RESPONSE
    //===========================================================================================================================

    Ok(Json(PredictResponse {
        output: final_output,
    }))
}