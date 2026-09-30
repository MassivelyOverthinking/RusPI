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

    #[allow(clippy::map_entry)]
    pub fn add(&mut self, input: &[f32], prediction: Vec<f32>) {
        let key = Self::make_key(input);

        if self.cache.contains_key(&key) {
            self.cache.insert(key, prediction);
            return;
        }

        if self.is_full()
            && let Some(key) = self.order.pop()
        {
            self.cache.remove(&key);
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

//===========================================================================================================================
// SIMPLE CACHE: Test
//===========================================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_cache_is_empty() {
        let test_cache: SimpleCache = SimpleCache::new();

        assert!(test_cache.get(&[1.0, 2.0]).is_none());
    }

    #[test]
    fn test_add_and_get_prediction() {
        let mut test_cache: SimpleCache = SimpleCache::new();

        let input = [1.0, 2.0];
        let prediction = vec![0.1, 0.2, 0.3];

        test_cache.add(&input, prediction.clone());

        assert_eq!(test_cache.get(&input), Some(&prediction));
    }

    #[test]
    fn test_missing_input_returns_none() {
        let mut test_cache: SimpleCache = SimpleCache::new();

        let input = [1.0, 2.0];
        let prediction = vec![0.1, 0.2, 0.3];

        test_cache.add(&input, prediction.clone());

        assert!(test_cache.get(&[2.0, 3.0]).is_none());
    }

    #[test]
    fn test_different_inputs() {
        let mut test_cache: SimpleCache = SimpleCache::new();

        let input_a = [1.0, 2.0];
        let input_b = [3.0, 4.0];

        let prediction_a = vec![0.1, 0.2, 0.3];
        let prediction_b = vec![0.4, 0.5, 0.6];

        test_cache.add(&input_a, prediction_a.clone());
        test_cache.add(&input_b, prediction_b.clone());

        assert_eq!(test_cache.get(&input_a), Some(&prediction_a));
        assert_eq!(test_cache.get(&input_b), Some(&prediction_b));
    }

    #[test]
    fn test_same_input_updates_value() {
        let mut test_cache: SimpleCache = SimpleCache::new();

        let input = [1.0, 2.0];

        let prediction_a = vec![0.1, 0.2, 0.3];
        let prediction_b = vec![0.4, 0.5, 0.6];

        test_cache.add(&input, prediction_a);
        test_cache.add(&input, prediction_b.clone());

        assert_eq!(test_cache.get(&input), Some(&prediction_b));
    }

    #[test]
    fn test_cache_evicts_most_recent_value() {
        let mut test_cache: SimpleCache = SimpleCache::new();

        for i in 0..100 {
            test_cache.add(&[i as f32], vec![i as f32]);
        }

        test_cache.add(&[100.0], vec![100.0]);

        assert!(test_cache.get(&[99.0]).is_none());
        assert_eq!(test_cache.get(&[0.0]), Some(&vec![0.0]));
        assert_eq!(test_cache.get(&[100.0]), Some(&vec![100.0]));
    }

    #[test]
    fn test_empty_input() {
        let mut test_cache: SimpleCache = SimpleCache::new();

        let input: [f32; 0] = [];
        let prediction = vec![1.0];

        test_cache.add(&input, prediction.clone());

        assert_eq!(test_cache.get(&input), Some(&prediction));
    }
}
