
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum VarType {
    Entero = 0,
    Flotante = 1,
    Boleano = 2,
}

pub const VAR_TYPE_COUNT: usize = 3;