pub mod operators;
pub mod var_types;
pub mod func_types;
pub mod type_matching;
pub mod dir_func;
pub mod semantic_error;

pub use type_matching::TypeMatching;
pub use dir_func::DirFunc;
pub use semantic_error::SemanticError;

pub struct Semantics {
    pub type_matching: TypeMatching,
    pub dir_func: DirFunc,
    errors: Vec::<SemanticError>,
}

impl Semantics {
    pub fn new() -> Self {
        Self {
            type_matching: TypeMatching::new(),
            dir_func: DirFunc::new(),
            errors: Vec::<SemanticError>::new(),
        }
    }

    pub fn get_errors(&self) -> &Vec::<SemanticError> {
        return &self.errors;
    }

}