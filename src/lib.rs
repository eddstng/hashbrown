use sha1::{ Digest, Sha1 };
use hex_literal::hex;

// https://www.reddit.com/r/rust/comments/18acuv4/how_to_define_a_trait_with_a_function_that/
trait Hasher<const N: usize> {
    fn update(&mut self, data: &[u8]);
    fn finalize(self) -> [u8; N];
}

struct Sha1Hasher {
    state: Sha1,
}

impl Sha1Hasher {
    fn new() -> Sha1Hasher {
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
        let fixed_array: [u8; 20] = result.try_into().expect("Failed to try_into fixed_array Sha1Hasher.finalize.");
        fixed_array
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha_1_hasher() {
        let mut hasher = Sha1Hasher::new();
        hasher.update(b"helloworld");
        let result = hasher.finalize();
        // https://emn178.github.io/online-tools/sha1.html
        assert_eq!(result[..], hex!("6adfb183a4a2c94a2f92dab5ade762a47889a5a1"));
    }
}
