//===========================================================================================================================
// IMPORTS
//===========================================================================================================================

use std::{sync::RwLock};
use ort::{session::{Session, builder::GraphOptimizationLevel}};

use crate::cache::cache::SimpleCache;

//===========================================================================================================================
// APP STATE
//===========================================================================================================================

pub struct AppState {
    pub session: RwLock<ort::session::Session>,
    pub cache: RwLock<SimpleCache>,
}

impl AppState {
    pub fn new() -> Result<Self, ort::error::Error> {
        let session = Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            .commit_from_file("")?;

        let cache = SimpleCache::new();

        Ok(Self {
            session: RwLock::new(session),
            cache: RwLock::new(cache),
        })
    }
}