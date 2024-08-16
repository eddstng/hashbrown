use sha1::{Digest, Sha1};

trait Hasher {
    fn update() -> String;
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
    fn update() -> String {
        return "".to_string();
    }

    fn finalize() -> String {
        return "".to_string();
    }
}
