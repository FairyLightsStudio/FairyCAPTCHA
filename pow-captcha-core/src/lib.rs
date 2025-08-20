use rand::Rng;
use sha2::{Digest, Sha256, Sha512};

const DEFAULT_DIFFICULTY: usize = 4;

#[derive(Clone,Debug,Default)]
pub enum HashAlgorithm {
    #[default]
    Sha256,
    Sha512
}

/// Represents a PoW challenge (the "puzzle").
/// This is sent to the client.
#[derive(Debug, Clone)]
pub struct Challenge {
    pub hash_algorithm: HashAlgorithm,
    /// The random string that needs to be hashed.
    pub puzzle: String,
    /// The number of leading zeros required in the hash.
    pub difficulty: usize,
}

/// Represents the solution to a PoW challenge, provided by the client.
#[derive(Debug, Clone)]
pub struct Solution {
    /// The nonce found by the client.
    pub nonce: u64,
}

/// Generates a new PoW challenge to be sent to the client.
///
/// # Arguments
///
/// * `difficulty` - An optional `usize` that specifies the number of leading zeros required in the hash.
///   If `None`, `DEFAULT_DIFFICULTY` (4) is used.
///
/// # Returns
///
/// A `Challenge` struct containing the puzzle and difficulty.
pub fn generate_challenge( difficulty: Option<usize>, hash_algorithm: Option<HashAlgorithm>) -> Challenge {
    let hash_algorithm = hash_algorithm.unwrap_or_default();
    let difficulty = difficulty.unwrap_or(DEFAULT_DIFFICULTY);
    let puzzle = generate_random_string(16);

    Challenge {
        hash_algorithm,
        puzzle,
        difficulty,
    }
}

/// Verifies a solution provided by a client.
///
/// # Arguments
///
/// * `challenge` - The original `Challenge` that was sent to the client.
/// * `solution` - The `Solution` submitted by the client.
///
/// # Returns
///
/// `true` if the solution is valid, `false` otherwise.
pub fn verify_solution(challenge: &Challenge, solution: &Solution) -> bool {
    let hash_hex = compute_hash(&challenge.hash_algorithm, &challenge.puzzle, solution.nonce);
    hash_hex.starts_with(&"0".repeat(challenge.difficulty))
}


/// Generates a random alphanumeric string of a given length.
fn generate_random_string(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Computes the hash for a given puzzle and nonce using the specified algorithm.
fn compute_hash(algorithm: &HashAlgorithm, puzzle: &str, nonce: u64) -> String {
    match algorithm {
        HashAlgorithm::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(puzzle.as_bytes());
            hasher.update(nonce.to_string().as_bytes());
            let result = hasher.finalize();
            hex::encode(result)
        }
        HashAlgorithm::Sha512 => {
            let mut hasher = Sha512::new();
            hasher.update(puzzle.as_bytes());
            hasher.update(nonce.to_string().as_bytes());
            let result = hasher.finalize();
            hex::encode(result)
        }
    }
}

/// Solves a given PoW challenge.
/// This function is computationally intensive and is intended to be run on the client side.
///
/// # Arguments
///
/// * `challenge` - The `Challenge` to solve.
///
/// # Returns
///
/// The `Solution` containing the correct nonce.
pub fn solve_challenge(challenge: &Challenge) -> Solution {
    let mut nonce = 0;
    loop {
        let hash_hex = compute_hash(&challenge.hash_algorithm, &challenge.puzzle, nonce);
        if hash_hex.starts_with(&"0".repeat(challenge.difficulty)) {
            return Solution { nonce };
        }
        nonce += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generates_challenge() {
        let challenge = generate_challenge(Some(5), None);
        assert_eq!(challenge.difficulty, 5);
        assert_eq!(challenge.puzzle.len(), 16);
    }

    #[test]
    fn it_uses_default_difficulty() {
        let challenge = generate_challenge(None, None);
        assert_eq!(challenge.difficulty, DEFAULT_DIFFICULTY);
    }

    #[test]
    fn it_verifies_a_correct_solution() {
        let challenge = generate_challenge(Some(4), None);
        // Simulate a client solving the challenge
        let solution = solve_challenge(&challenge);
        
        assert!(verify_solution(&challenge, &solution));
    }

    #[test]
    fn it_rejects_an_incorrect_solution() {
        let challenge = generate_challenge(Some(4), None);
        // Simulate a client providing a wrong nonce
        let incorrect_solution = Solution { nonce: 12345 }; // An arbitrary, likely incorrect nonce
        
        // We need to ensure the incorrect nonce is actually incorrect
        let mut hasher = Sha256::new();
        hasher.update(challenge.puzzle.as_bytes());
        hasher.update(incorrect_solution.nonce.to_string().as_bytes());
        let result = hasher.finalize();
        let hash_hex = hex::encode(result);

        if !hash_hex.starts_with(&"0".repeat(challenge.difficulty)) {
             assert!(!verify_solution(&challenge, &incorrect_solution));
        } else {
            // In the very unlikely event that our random incorrect nonce was correct,
            // we just pass the test. This is statistically negligible.
            println!("Warning: Random incorrect nonce happened to be correct.");
        }
    }
}
