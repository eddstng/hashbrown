use sha1::{Digest, Sha1};

trait Hasher {
    fn update(&mut self, data: &[u8]);
    fn finalize() -> String;
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

impl Hasher for Sha1Hasher {
    fn update(&mut self, data: &[u8]) {
        // https://docs.rs/sha1/latest/sha1/trait.Digest.html#tymethod.update
        self.state.update(data);
    }

    fn finalize() -> String {
        return "".to_string();
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_sha_1_hasher() {
//         let mut hasher = Sha1Hasher::new();
//         hasher.update(b"helloworld");
//     }
// }