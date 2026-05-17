use crate::scanner::Token;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum VarType {
    Entero = 0,
    Flotante = 1,
}