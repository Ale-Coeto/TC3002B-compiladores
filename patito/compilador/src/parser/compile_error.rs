
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum ErrorType {
    Lexic = 1,
    Sintactic = 2,
    Semantic = 3,
    Quad = 4,
}

#[derive(Debug, PartialEq)]
pub struct CompileError {
    pub error_type: ErrorType,
    pub message: String,
}