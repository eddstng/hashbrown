use crate::hasher;

use hasher::Hasher;
use sha1::{ Digest, Sha1 };

pub struct Sha1Hasher {
    state: Sha1,
}

impl Sha1Hasher {
    pub fn new() -> Sha1Hasher {
        Sha1Hasher {
            state: Sha1::new(),
        }
    }
}

impl Hasher<20> for Sha1Hasher {
    fn update(&mut self, data: &[u8]) {
        // https://docs.rs/sha1/latest/sha1/trait.Digest.html#tymethod.update
        self.state.update(data);
    }

    fn finalize(self) -> [u8; 20] {
        // https://docs.rs/sha1/latest/sha1/trait.Digest.html#tymethod.finalize
        let result = self.state.finalize();
        let fixed_array: [u8; 20] = result
            .try_into()
            .expect("Failed to try_into fixed_array Sha1Hasher.finalize.");
        fixed_array
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_sha_1_hasher() {
        let mut hasher = Sha1Hasher::new();
        hasher.update(b"helloworld");
        let result = hasher.finalize();
        // https://emn178.github.io/online-tools/sha1.html
        let expected: [u8; 20] = hex!("6adfb183a4a2c94a2f92dab5ade762a47889a5a1");
        assert_eq!(result[..], expected, "The hash did not match the empty hex value");
    }

    #[test]
    fn test_sha_1_hasher_empty() {
        let mut hasher = Sha1Hasher::new();
        hasher.update(b"");
        let result = hasher.finalize();
        // https://emn178.github.io/online-tools/sha1.html
        let expected: [u8; 20] = hex!("da39a3ee5e6b4b0d3255bfef95601890afd80709");
        assert_eq!(result, expected, "The hash did not match the empty hex value");
    }
}
