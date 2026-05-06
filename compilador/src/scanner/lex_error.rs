#[derive(Debug, PartialEq)]
pub struct LexError {
    pub start: usize,
    pub end: usize,
}