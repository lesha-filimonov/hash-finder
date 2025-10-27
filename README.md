# Hash Finder

A Rust console application that finds SHA-256 hashes of consecutive integers (starting from 1) that end with `N` zeros, until `F` such hashes are found.

## Features

- Calculates SHA-256 hashes using the `sha2` crate.
- Uses threads for parallel processing to improve performance.
- Prints hash-number pairs as soon as they are found.
- Includes unit tests for hash calculation and trailing zeros detection.
- Modularized codebase for maintainability.

## Project Structure

- `src/main.rs`: Entry point for the application.
- `src/lib.rs`: Declares modules and contains unit tests.
- `src/args.rs`: Command-line argument parsing.
- `src/hash.rs`: SHA-256 hash computation and trailing zeros checking.
- `src/finder.rs`: Parallel hash finding logic.

## Prerequisites

- [Rust](https://rustup.rs/)
- Git

## Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/lesha-filimonov/hash-finder.git
   cd hash_finder
   ```

2. Build the project:
    ```bash
    cargo build --release
    ```

3. Or run the project with cargo:
    ```bash
    cargo run -- -N <number_of_zeros> -F <number_of_results>
    ```

## Usage

Run the program with the -N and -F flags:
```bash
./target/release/hash_finder -N <number_of_zeros> -F <number_of_results>
```
Example:
```bash
./target/release/hash_finder -N 3 -F 6
4163, "95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000"
11848, "cb58074fd7620cd0ff471922fd9df8812f29f302904b15e389fc14570a66f000"
12843, "bb90ff93a3ee9e93c123ebfcd2ca1894e8994fef147ad81f7989eccf83f64000"
13467, "42254207576dd1cfb7d0e4ceb1afded40b5a46c501e738159d8ac10b36039000"
20215, "1f463eb31d6fa7f3a7b37a80f9808814fc05bf10f01a3f653bf369d7603c8000"
28892, "dab12874ecae90c0f05d7d87ed09921b051a586c7321850f6bb5e110bc6e2000"
```

## Running Tests

The project includes unit tests for hash calculation and trailing zeros detection. To run the tests:
```bash
cargo test
```
