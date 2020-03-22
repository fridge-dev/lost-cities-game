use std::sync::{Arc, Mutex};
use game_api::api::GameApi2;
use crate::backend::backend_error::BackendGameError2;
use crate::backend::backend_impl;
use backend_engine::task::backend_task_client;
use backend_engine::backend_error::BackendGameError;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use backend_engine::task::backend_task_client::BackendTaskClient;

pub fn new_backend_game_api() -> Arc<Mutex<dyn GameApi2<BackendGameError2> + Send>> {
    Arc::new(Mutex::new(backend_impl::StorageBackedGameApi::new()))
}

pub fn start_backend() -> Slots<BackendTaskClient> {
    let num_cpus = num_cpus::get();

    println!("There are {} CPUs. Spawning {} backend tasks.", num_cpus, num_cpus);

    let mut task_clients = Vec::with_capacity(num_cpus);
    for _ in 0..num_cpus {
        task_clients.push(backend_task_client::spawn_backend_task());
    }

    Slots {
        slots: task_clients
    }
}

/// Encapsulates a finite key-space of values accessible via an infinite key-space
/// thanks to consistent hashing.
pub struct Slots<V> {
    slots: Vec<V>,
}

impl<V> Slots<V> {
    pub fn get<K: Hash>(&self, key: &K) -> &V {
        let index = consistent_hash_index(key, self.slots.len());

        self.slots
            .get(index)
            .expect("consistent hash resulted in out-of-bounds index")
    }
}

fn consistent_hash_index<K: Hash>(key: K, modulo: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    let hash = hasher.finish();

    (hash as usize) % modulo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consistent_hash_is_consistent() {
        let h1 = consistent_hash_index("asdf", 4);
        let h2 = consistent_hash_index("asdf", 4);

        assert_eq!(h1, h2);
    }

    #[test]
    fn consistent_hash_index_in_bounds() {
        for i in 0..10000 {
            assert!(consistent_hash_index(i, 5) < 5);
        }
    }

    #[test]
    fn consistent_hash_evenly_distributes_keys() {
        // Feel free to change these values when changing hash algorithm, as needed.
        let num_slots = 8;
        let num_keys_to_hash = 10000;
        let min_num_keys_per_slot = 1200; // (10,000/8) = 1,250
        let max_peak_to_avg_ratio = 1.2f32;

        let mut keys_per_slot: Vec<u32> = Vec::with_capacity(num_slots);
        for _ in 0..num_slots {
            keys_per_slot.push(0);
        }

        for key in 0..num_keys_to_hash {
            let index = consistent_hash_index(key, num_slots);
            let value = keys_per_slot.get_mut(index).unwrap();
            *value += 1;
        }

        println!("Num keys per slot: {:?}", keys_per_slot);

        for i in 0..num_slots {
            assert!(*keys_per_slot.get(i).unwrap() >= min_num_keys_per_slot);
        }

        // Peak-to-avg ratio:
        let max: f32 = *keys_per_slot.iter().max().unwrap() as f32;
        let avg: f32 = keys_per_slot.iter().sum::<u32>() as f32 / (keys_per_slot.len() as f32);
        println!("Peak to avg: {} : {}", max, avg);

        assert!(max / avg < max_peak_to_avg_ratio);
    }
}
