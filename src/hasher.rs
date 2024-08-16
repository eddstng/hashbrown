// the Hasher trait is a generic interface that takes in a usize based on the hash length
pub trait Hasher<const N: usize> {
    fn update(&mut self, data: &[u8]); // updates the state of the hashing
    fn finalize(self) -> [u8; N]; // end hashing process and returns hash as a fixed-sized array
}
