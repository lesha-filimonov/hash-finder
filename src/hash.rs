use sha2::{Digest, Sha256};

pub fn compute_sha256(num: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(num.to_string().as_bytes());
    let hash = hasher.finalize();
    hex::encode(hash)
}
