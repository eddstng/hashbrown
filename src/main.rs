use hasher::{ Hasher, Sha1Hasher, Sha256Hasher };

fn main() {
    println!("\nWelcome to Hashbrown!");
    println!("A simple Rust library for hashing data using SHA-1 and SHA-256 algorithms.\n");

    let mut sha1_hasher = Sha1Hasher::new();
    sha1_hasher.update(b"helloworld");
    let sha1_result = sha1_hasher.finalize();
    println!("SHA-1 hash of 'helloworld':");
    println!("{:x?}", sha1_result);
    println!();

    let mut sha256_hasher = Sha256Hasher::new();
    sha256_hasher.update(b"helloworld");
    let sha256_result = sha256_hasher.finalize();
    println!("SHA-256 hash of 'helloworld':");
    println!("{:x?}", sha256_result);
    println!();
}
