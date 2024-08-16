# Hashbrown

A simple Rust library for hashing data using SHA-1 and SHA-256 algorithms. 

![Logo](https://i.imgur.com/R3fxDRm.png)

## Running Tests

To run the tests, run the following commands

```bash
$ cargo test
```

## Running Examples

To run the examples, run the following commands

```bash
$ cargo run
```

## Usage
```rust
let mut hasher = Sha1Hasher::new();
hasher.update(b"helloworld");
let result = hasher.finalize();
let expected: [u8; 20] = hex!(
    "6adfb183a4a2c94a2f92dab5ade762a47889a5a1"
);
assert_eq!(result[..], expected);
```
```rust
let mut hasher = Sha256Hasher::new();
hasher.update(b"helloworld");
let result = hasher.finalize();
let expected: [u8; 32] = hex!(
    "936a185caaa266bb9cbe981e9e05cb78cd732b0b3280eb944412bb6f8f8f07af"
);
assert_eq!(result, expected);
```

## Considerations for the Future

- This design was built to support environments without an embedded memory allocator but can also be used in environments with heap memory. If redesigned specifically for environments with heap memory allocators, we could use the Vec<u8> type. This would allow us to unify the current hashing structs into one, with the hash length determined dynamically, providing more flexibility.

- Currently, the code uses expect for error handling, which should never be triggered given the expected hash lengths of the current algorithms. However, in an embedded environment, we need to be cautious about using panics and should implement a more appropriate error-handling mechanism.