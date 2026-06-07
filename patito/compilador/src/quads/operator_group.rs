#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OperatorGroup {
    SumSub,
    MultDiv,
    Rel,
}

impl OperatorGroup {
    pub fn matches(&self, op: crate::quads::quad_operators::QuadOperator) -> bool {
        use crate::quads::quad_operators::QuadOperator::*;

        match self {
            OperatorGroup::SumSub => matches!(op, Mas | Menos),
            OperatorGroup::MultDiv => matches!(op, Multiplicar | Dividir),
            OperatorGroup::Rel => matches!(op, ComparadorIgual | NoIgual | Mayor | Menor),
        }
    }
}
