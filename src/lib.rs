use sha1::{ Digest, Sha1 };

trait Hasher {
    fn update(&mut self, data: &[u8]);
    fn finalize(self) -> String;
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

    fn finalize(self) -> String {
        // https://docs.rs/sha1/latest/sha1/trait.Digest.html#tymethod.finalize
        let result: sha1::digest::generic_array::GenericArray<
            u8,
            sha1::digest::typenum::UInt<
                sha1::digest::typenum::UInt<
                    sha1::digest::typenum::UInt<
                        sha1::digest::typenum::UInt<
                            sha1::digest::typenum::UInt<
                                sha1::digest::typenum::UTerm,
                                sha1::digest::consts::B1
                            >,
                            sha1::digest::consts::B0
                        >,
                        sha1::digest::consts::B1
                    >,
                    sha1::digest::consts::B0
                >,
                sha1::digest::consts::B0
            >
        > = self.state.finalize();
        println!("{:x}", result);
        return format!("{:x}", result);
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
        assert_eq!(result, "6adfb183a4a2c94a2f92dab5ade762a47889a5a1")
    }
}
