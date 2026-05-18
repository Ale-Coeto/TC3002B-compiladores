#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum FuncType {
    Entero = 0,
    Flotante = 1,
    Nula = 2,
    Program = 3,
}