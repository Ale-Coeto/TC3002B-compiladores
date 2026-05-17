pub mod operators;
pub mod var_types;
pub mod type_matching;

pub use type_matching::TypeMatching;

pub struct Semantics {
    type_matching: TypeMatching,
}

impl Semantics {
    pub fn new() -> Self {
        Self {
            type_matching: TypeMatching::new(),
        }
    }
}