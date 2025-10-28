use crate::hash::{compute_sha256, has_trailing_zeros};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;

#[derive(Debug, Clone)]
pub struct HashResult {
    pub number: u64,
    pub hash: String,
}

pub fn find_hashes(target_zeros: usize, target_count: usize) -> Result<Vec<HashResult>, String> {
    let should_stop = Arc::new(AtomicBool::new(false));
    let counter = Arc::new(AtomicU64::new(1));
    let found_count = Arc::new(AtomicU64::new(0));

    let num_threads = num_cpus::get().max(1);
    let mut handles = Vec::with_capacity(num_threads);

    for _ in 0..num_threads {
        let should_stop = Arc::clone(&should_stop);
        let counter = Arc::clone(&counter);
        let found_count = Arc::clone(&found_count);

        let handle = thread::spawn(move || {
            let mut local_results = Vec::new();

            while !should_stop.load(Ordering::Relaxed) {
                let num = counter.fetch_add(1, Ordering::Relaxed);
                let hash = compute_sha256(num);

                if has_trailing_zeros(&hash, target_zeros) {
                    local_results.push((num, hash));

                    let new_count = found_count.fetch_add(1, Ordering::Relaxed) + 1;
                    if new_count >= target_count as u64 {
                        should_stop.store(true, Ordering::Relaxed);
                    }
                }
            }
            local_results
        });

        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        match handle.join() {
            Ok(local_results) => results.extend(local_results),
            Err(_) => return Err("Thread panicked".to_string()),
        }
    }

    results.truncate(target_count);

    let results_vec = results
        .into_iter()
        .map(|(number, hash)| HashResult { number, hash })
        .collect();

    Ok(results_vec)
}
