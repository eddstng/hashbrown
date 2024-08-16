// https://www.reddit.com/r/rust/comments/18acuv4/how_to_define_a_trait_with_a_function_that/
pub trait Hasher<const N: usize> {
    fn update(&mut self, data: &[u8]);
    fn finalize(self) -> [u8; N];
}
