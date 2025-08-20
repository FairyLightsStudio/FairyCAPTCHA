# PoW Captcha Core

`pow-captcha-core` is a simple, lightweight, and dependency-free Proof-of-Work (PoW) CAPTCHA library for Rust. It provides the core logic for generating and verifying PoW challenges.

## Features

- Generate PoW challenges with a specified difficulty.
- Verify solutions provided by clients.
- Solve challenges (intended for client-side use).
- Supports SHA-256 and SHA-512 hashing algorithms.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
pow-captcha-core = "0.1.0"
```

### Example

```rust
use pow_captcha_core::{generate_challenge, solve_challenge, verify_solution, HashAlgorithm};

// 1. Server: Generate a challenge
let challenge = generate_challenge(Some(4), Some(HashAlgorithm::Sha256));

// 2. Client: Solve the challenge
let solution = solve_challenge(&challenge);

// 3. Server: Verify the solution
assert!(verify_solution(&challenge, &solution));
```

## License

This project is licensed under the MIT License.
