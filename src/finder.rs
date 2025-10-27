use crate::hash::{compute_sha256, has_trailing_zeros};
use std::io::{self, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;

pub fn find_hashes(target_zeros: usize, target_count: usize) {
    let should_stop = Arc::new(AtomicBool::new(false));
    let counter = Arc::new(AtomicU64::new(1));
    let results = Arc::new(std::sync::Mutex::new(Vec::new()));

    let num_threads = num_cpus::get();
    let mut handles = Vec::with_capacity(num_threads);

    for _ in 0..num_threads {
        let should_stop = Arc::clone(&should_stop);
        let counter = Arc::clone(&counter);
        let results = Arc::clone(&results);

        let handle = thread::spawn(move || {
            while !should_stop.load(Ordering::Relaxed) {
                let num = counter.fetch_add(1, Ordering::Relaxed);
                let hash = compute_sha256(num);

                if has_trailing_zeros(&hash, target_zeros) {
                    let mut results_guard = results.lock().unwrap();

                    if results_guard.len() >= target_count {
                        should_stop.store(true, Ordering::Relaxed);
                        break;
                    }

                    println!("{}, \"{}\"", num, hash);
                    let _ = io::stdout().flush();

                    results_guard.push((num, hash));

                    if results_guard.len() >= target_count {
                        should_stop.store(true, Ordering::Relaxed);
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }
}
