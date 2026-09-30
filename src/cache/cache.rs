//===========================================================================================================================
// IMPORTS
//===========================================================================================================================

use std::collections::HashMap;

//===========================================================================================================================
// SIMPLE CACHE
//===========================================================================================================================

pub struct SimpleCache {
    capacity: u8,
    order: Vec<Vec<u8>>,
    cache: std::collections::HashMap<Vec<u8>, Vec<f32>>,
}

impl SimpleCache {
    pub fn new() -> Self {
        Self {
            capacity: 100,
            order: Vec::new(),
            cache: HashMap::new(),
        }
    }

    pub fn get(&self, input: &[f32]) -> Option<&Vec<f32>> {
        let key = Self::make_key(input);

        self.cache.get(&key)
    }

    pub fn add(&mut self, input: &[f32], prediction: Vec<f32>) {
        let key = Self::make_key(input);

        if self.cache.contains_key(&key) {
            self.cache.insert(key, prediction);
            return;
        }

        if self.is_full() {
            if let Some(key) = self.order.pop() {
                self.cache.remove(&key);
            }
        }

        self.cache.insert(key.clone(), prediction);
        self.order.push(key);
    }

    fn is_full(&self) -> bool {
        self.cache.len() >= self.capacity.into()
    }

    fn make_key(input: &[f32]) -> Vec<u8> {
        input.iter().flat_map(|value| value.to_ne_bytes()).collect()
    }
}
