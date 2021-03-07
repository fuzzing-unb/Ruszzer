pub trait Fuzzer {
    fn fuzz(&self, max_length: usize, char_start: u8, char_range: u8) -> String;
}