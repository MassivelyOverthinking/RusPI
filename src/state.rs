//===========================================================================================================================
// IMPORTS
//===========================================================================================================================

use std::collections::HashMap;
use ort::session::{Session, builder::GraphOptimizationLevel};

//===========================================================================================================================
// APP STATE
//===========================================================================================================================

pub struct AppState {
    pub session: ort::session::Session,
    pub cache: std::collections::HashMap<String, String>,
}

impl AppState {
    pub fn new() -> Result<Self, ort::error::Error> {
        let session = Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            .commit_from_file("")?;

        let cache = HashMap::new();

        Ok(Self {
            session,
            cache,
        })
    }
}