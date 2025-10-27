use crate::hash::{compute_sha256, has_trailing_zeros};
use std::io::{self, Write};

pub fn find_hashes(target_zeros: usize, target_count: usize) {
    let mut found_count = 0;
    let mut results = Vec::new();

    for num in 1.. {
        if found_count >= target_count {
            break;
        }

        let hash = compute_sha256(num);
        if has_trailing_zeros(&hash, target_zeros) {
            results.push((num, hash.clone()));
            found_count += 1;

            println!("{}, \"{}\"", num, hash);
            io::stdout().flush().unwrap();
        }
    }
}
