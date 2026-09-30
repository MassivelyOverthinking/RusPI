//===========================================================================================================================
// IMPORTS
//===========================================================================================================================

use serde::{Deserialize, Serialize};

//===========================================================================================================================
// RESPONSE
//===========================================================================================================================

#[derive(Debug, Deserialize)]
pub struct PredictRequest {
    pub input: Vec<f32>,
}

#[derive(Debug, Serialize)]
pub struct PredictResponse {
    pub output: Vec<f32>,
}

//===========================================================================================================================
// RESPONSE: Test
//===========================================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_deserialization() {
        let test_json_str = r#"{
            "input": [1.0, 2.0, 3.0]
        }"#;

        let test_request: PredictRequest =
            serde_json::from_str(test_json_str).expect("Failed to deserialize");

        assert_eq!(test_request.input, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_response_serialization() {
        let test_response = PredictResponse {
            output: vec![0.1, 0.2, 0.3],
        };

        let test_json_str = serde_json::to_string(&test_response).expect("Failed to serialize");

        assert_eq!(test_json_str, r#"{"output":[0.1,0.2,0.3]}"#);
    }

    #[test]
    fn test_request_handles_empty_input() {
        let test_json_str = r#"{ 
            "input": [] 
        }"#;

        let test_request: PredictRequest =
            serde_json::from_str(test_json_str).expect("Failed to deserialize");

        assert!(test_request.input.is_empty());
    }

    #[test]
    fn test_response_handles_empty_input() {
        let test_response = PredictResponse { output: vec![] };

        let test_json_str = serde_json::to_string(&test_response).expect("Failed to serialize");

        assert_eq!(test_json_str, r#"{"output":[]}"#);
    }
}
