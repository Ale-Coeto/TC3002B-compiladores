#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Operator {
    Mas = 0,
    Menos = 1,
    Multiplicar = 2,
    Dividir = 3,
    ComparadorIgual = 4,
    NoIgual = 5,
    Mayor = 6,
    Menor = 7,
}

pub const OP_COUNT: usize = 8;