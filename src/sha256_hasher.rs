use crate::hasher;

use hasher::Hasher;
use sha2::{ Digest, Sha256 };

pub struct Sha256Hasher {
    state: Sha256,
}

impl Sha256Hasher {
    pub fn new() -> Sha256Hasher {
        Sha256Hasher {
            state: Sha256::new(),
        }
    }
}

impl Hasher<32> for Sha256Hasher {
    fn update(&mut self, data: &[u8]) {
        // https://docs.rs/sha2/0.10.8/sha2/trait.Digest.html#tymethod.update
        self.state.update(data);
    }

    fn finalize(self) -> [u8; 32] {
        // https://docs.rs/sha2/0.10.8/sha2/trait.Digest.html#tymethod.finalize
        let result = self.state.finalize();
        let fixed_array: [u8; 32] = result
            .try_into()
            .expect("Failed to try_into fixed_array Sha256Hasher.finalize.");
        fixed_array
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_sha_256_hasher() {
        let mut hasher = Sha256Hasher::new();
        hasher.update(b"helloworld");
        let result = hasher.finalize();
        // https://emn178.github.io/online-tools/sha256.html
        let expected: [u8; 32] = hex!(
            "936a185caaa266bb9cbe981e9e05cb78cd732b0b3280eb944412bb6f8f8f07af"
        );
        assert_eq!(result, expected, "The hash did not match the expected hex value");
    }

    #[test]
    fn test_sha_256_hasher_empty() {
        let mut hasher = Sha256Hasher::new();
        hasher.update(b"");
        let result = hasher.finalize();
        // https://emn178.github.io/online-tools/sha256.html
        let expected: [u8; 32] = hex!(
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(result, expected, "The hash did not match the empty hex value");
    }
}
