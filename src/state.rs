//===========================================================================================================================
// IMPORTS
//===========================================================================================================================

use ort::session::{Session, builder::GraphOptimizationLevel};
use std::sync::RwLock;

use crate::cache::storage::SimpleCache;

//===========================================================================================================================
// APP STATE
//===========================================================================================================================

pub struct AppState {
    pub session: RwLock<ort::session::Session>,
    pub cache: RwLock<SimpleCache>,
}

impl AppState {
    pub fn new() -> Result<Self, ort::error::Error> {
        let model_path = concat!(env!("CARGO_MANIFEST_DIR"), "/onnx/resnet18-v1-7.onnx");

        let session = Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            .commit_from_file(model_path)?;

        let cache = SimpleCache::new();

        Ok(Self {
            session: RwLock::new(session),
            cache: RwLock::new(cache),
        })
    }
}

//===========================================================================================================================
// APP STATE: Test
//===========================================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_initialization() {
        let test_state = AppState::new();

        assert!(
            test_state.is_ok(),
            "AppState failed to initialize {:?}",
            test_state.err()
        );
    }

    #[test]
    fn test_app_state_contains_session() {
        let test_state = AppState::new().expect("AppState failed to initialize");

        let test_session = test_state
            .session
            .read()
            .expect("Failed to acquire RwLoack from session");

        assert!(
            !test_session.inputs().is_empty(),
            "ONNX session should contain input"
        );

        assert!(
            !test_session.outputs().is_empty(),
            "ONNX session should contain output"
        );
    }

    #[test]
    fn test_app_state_contains_empty_cache() {
        let test_state = AppState::new().expect("AppState failed to initialize");

        let test_cache = test_state
            .cache
            .read()
            .expect("Failed to acquire RwLock from cache");

        assert!(
            test_cache.get(&[1.0, 2.0]).is_none(),
            "Cache should be empty at startup"
        )
    }

    #[test]
    fn test_app_state_locks_read_and_write() {
        let test_state = AppState::new().expect("AppState failed to initialize");

        {
            let test_cache = test_state
                .cache
                .read()
                .expect("Failed to acquire lock for cache");

            assert!(test_cache.get(&[1.0, 2.0]).is_none());
        }

        {
            let mut test_cache = test_state.cache.write().expect("Failed to write to cache");

            test_cache.add(&[1.0], vec![2.0]);
        }

        {
            let test_cache = test_state
                .cache
                .read()
                .expect("Failed to acquire lock for cache");

            assert_eq!(test_cache.get(&[1.0]), Some(&vec![2.0]));
        }
    }
}
