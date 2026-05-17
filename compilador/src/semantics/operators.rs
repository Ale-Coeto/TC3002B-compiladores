use crate::scanner::Token;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Operator {
    Mas = 0,
    Menos = 1,
    Multiplicar = 2,
    Dividir = 3,
}