pub mod args;
pub mod hash;

#[cfg(test)]
mod tests {
    use super::hash::compute_sha256;

    #[test]
    fn test_compute_sha256_known_values() {
        let test_cases = vec![
            (
                1,
                "6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b",
            ),
            (
                4163,
                "95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000",
            ),
            (
                11848,
                "cb58074fd7620cd0ff471922fd9df8812f29f302904b15e389fc14570a66f000",
            ),
        ];

        for (num, expected_hash) in test_cases {
            let hash = compute_sha256(num);
            assert_eq!(
                hash, expected_hash,
                "Hash for {} does not match expected value",
                num
            );
        }
    }
}
