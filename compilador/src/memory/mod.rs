use crate::semantics::var_types::VarType;
use crate::quads::quad_value::QuadValue;

pub const INT_MEMORY_SIZE: usize = 500;
pub const FLOAT_MEMORY_SIZE: usize = 500;
pub const BOOL_MEMORY_SIZE: usize = 500;

pub struct MemoryManager {
    int_memory: [i64; INT_MEMORY_SIZE],
    float_memory: [f64; FLOAT_MEMORY_SIZE],
    bool_memory: [bool; BOOL_MEMORY_SIZE],

    int_memory_index: i64,
    float_memory_index: i64,
    bool_memory_index: i64,
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            int_memory: [0; INT_MEMORY_SIZE],
            float_memory: [0.0; FLOAT_MEMORY_SIZE],
            bool_memory: [false; BOOL_MEMORY_SIZE],

            int_memory_index: 0,
            float_memory_index: 0,
            bool_memory_index: 0,
        }
    }

    pub fn save_in_memory(&mut self, value: QuadValue) -> i64 {
        match value {
            QuadValue::Entero(val) => {
                let address = self.int_memory_index;
                self.int_memory[address as usize] = val;
                self.int_memory_index += 1;
                address
            }
            QuadValue::Flotante(val) => {
                let address = self.float_memory_index;
                self.float_memory[address as usize] = val;
                self.float_memory_index += 1;
                (INT_MEMORY_SIZE as i64) + address
            }
            QuadValue::Boleano(val) => {
                let address = self.bool_memory_index;
                self.bool_memory[address as usize] = val;
                self.bool_memory_index += 1;
                (INT_MEMORY_SIZE + FLOAT_MEMORY_SIZE) as i64 + address
            }
        }
    }

    pub fn get_from_memory(&self, address: i64) -> QuadValue {
        let int_limit = INT_MEMORY_SIZE as i64;
        let float_limit = int_limit + (FLOAT_MEMORY_SIZE as i64);
        let bool_limit = float_limit + (BOOL_MEMORY_SIZE as i64);

        match address {
            addr if addr >= 0 && addr < int_limit => {
                let index = addr as usize;
                QuadValue::Entero(self.int_memory[index])
            }
            addr if addr >= int_limit && addr < float_limit => {
                let index = (addr - int_limit) as usize;
                QuadValue::Flotante(self.float_memory[index])
            }
            addr if addr >= float_limit && addr < bool_limit => {
                let index = (addr - float_limit) as usize;
                QuadValue::Boleano(self.bool_memory[index])
            }
            _ => panic!("Error en tiempo de ejecución: Dirección de memoria {} fuera de rango", address),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_multiple_types_and_addresses() {
        let mut mm = MemoryManager::new();

        // two ints
        let a1 = mm.save_in_memory(QuadValue::Entero(10));
        let a2 = mm.save_in_memory(QuadValue::Entero(20));
        assert_eq!(a1, 0);
        assert_eq!(a2, 1);
        match mm.get_from_memory(a1) {
            QuadValue::Entero(v) => assert_eq!(v, 10),
            _ => panic!("expected Entero from memory"),
        }
        match mm.get_from_memory(a2) {
            QuadValue::Entero(v) => assert_eq!(v, 20),
            _ => panic!("expected Entero from memory"),
        }

        // two floats
        let f1 = mm.save_in_memory(QuadValue::Flotante(1.5));
        let f2 = mm.save_in_memory(QuadValue::Flotante(2.5));
        assert_eq!(f1, (INT_MEMORY_SIZE as i64) + 0);
        assert_eq!(f2, (INT_MEMORY_SIZE as i64) + 1);
        match mm.get_from_memory(f1) {
            QuadValue::Flotante(v) => assert_eq!(v, 1.5),
            _ => panic!("expected Flotante from memory"),
        }
        match mm.get_from_memory(f2) {
            QuadValue::Flotante(v) => assert_eq!(v, 2.5),
            _ => panic!("expected Flotante from memory"),
        }

        // two bools
        let b1 = mm.save_in_memory(QuadValue::Boleano(true));
        let b2 = mm.save_in_memory(QuadValue::Boleano(false));
        assert_eq!(b1, (INT_MEMORY_SIZE + FLOAT_MEMORY_SIZE) as i64 + 0);
        assert_eq!(b2, (INT_MEMORY_SIZE + FLOAT_MEMORY_SIZE) as i64 + 1);
        match mm.get_from_memory(b1) {
            QuadValue::Boleano(v) => assert_eq!(v, true),
            _ => panic!("expected Boleano from memory"),
        }
        match mm.get_from_memory(b2) {
            QuadValue::Boleano(v) => assert_eq!(v, false),
            _ => panic!("expected Boleano from memory"),
        }
    }
}