#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]

pub enum QuadOperator {
    Mas = 0,
    Menos = 1,
    Multiplicar = 2,
    Dividir = 3,
    ComparadorIgual = 4,
    NoIgual = 5,
    Mayor = 6,
    Menor = 7,
    Asignacion = 8,
    Imprime = 9,
    GoTo = 10,
    GoToV = 11,
    GoToF = 12,
    OpenParenthesis = 13,
    CloseParenthesis = 14,
}

pub const OP_COUNT: usize = 8;

impl TryFrom<QuadOperator> for crate::semantics::operators::Operator {
    type Error = ();

    fn try_from(q: QuadOperator) -> Result<Self, Self::Error> {
        use crate::semantics::operators::Operator;

        match q {
            QuadOperator::Mas => Ok(Operator::Mas),
            QuadOperator::Menos => Ok(Operator::Menos),
            QuadOperator::Multiplicar => Ok(Operator::Multiplicar),
            QuadOperator::Dividir => Ok(Operator::Dividir),
            QuadOperator::ComparadorIgual => Ok(Operator::ComparadorIgual),
            QuadOperator::NoIgual => Ok(Operator::NoIgual),
            QuadOperator::Mayor => Ok(Operator::Mayor),
            QuadOperator::Menor => Ok(Operator::Menor),
            _ => Err(()),
        }
    }
}