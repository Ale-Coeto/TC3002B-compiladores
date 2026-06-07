pub mod memory_error;

use crate::quads::quad_value::QuadValue;
use crate::semantics::var_types::VarType;
use crate::memory::memory_error::MemoryError;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub const GLOBAL_INT_START: i64 = 1000;
pub const GLOBAL_FLOAT_START: i64 = 1500;
pub const LOCAL_INT_START: i64 = 2000;
pub const LOCAL_FLOAT_START: i64 = 2500;
pub const TEMP_INT_START: i64 = 3000;
pub const TEMP_FLOAT_START: i64 = 4000;
pub const TEMP_BOOL_START: i64 = 5000;
pub const CONSTANTS_START: i64 = 6000;

const GLOBAL_LOCAL_MEMORY_SIZE: i64 = 500;
const TEMP_MEMORY_SIZE: i64 = 1000;

pub struct MemoryManager {
    int_global_index: i64,
    int_local_index: i64,
    float_global_index: i64,
    float_local_index: i64,
    int_temp_index: i64,
    float_temp_index: i64,
    bool_temp_index: i64,
    constants_index: i64,
    int_constants_dir: HashMap<i64, i64>,
    float_constants_dir: HashMap<u64, i64>,
}

pub enum ConstantValue {
    Entero(i64),
    Flotante(f64),
}

pub struct ConstantJson {
    pub address: i64,
    pub value: ConstantValue,
}

static MEMORY_MANAGER: OnceLock<Mutex<MemoryManager>> = OnceLock::new();

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            int_global_index: 0,
            int_local_index: 0,
            float_global_index: 0,
            float_local_index: 0,
            int_temp_index: 0,
            float_temp_index: 0,
            bool_temp_index: 0,
            constants_index: 0,
            int_constants_dir: HashMap::new(),
            float_constants_dir: HashMap::new(),
        }
    }

    pub fn instance() -> &'static Mutex<MemoryManager> {
        MEMORY_MANAGER.get_or_init(|| Mutex::new(MemoryManager::new()))
    }

    pub fn with_instance<R>(f: impl FnOnce(&mut MemoryManager) -> R) -> R {
        let mut guard = Self::instance()
            .lock()
            .expect("MemoryManager mutex poisoned");
        f(&mut guard)
    }

    pub fn reset(&mut self) {
        self.int_global_index = 0;
        self.int_local_index = 0;
        self.float_global_index = 0;
        self.float_local_index = 0;
        self.int_temp_index = 0;
        self.float_temp_index = 0;
        self.bool_temp_index = 0;
        self.constants_index = 0;
        self.int_constants_dir.clear();
        self.float_constants_dir.clear();
    }

    pub fn reset_function_scope(&mut self) {
        self.int_local_index = 0;
        self.float_local_index = 0;
        self.int_temp_index = 0;
        self.float_temp_index = 0;
        self.bool_temp_index = 0;
    }

    fn next_address(start: i64, index: &mut i64, limit: i64, label: &str) -> Result<i64, MemoryError> {
        if *index >= limit {
            return Err(MemoryError {
                message: format!("Límite de memoria para variables {} alcanzado", label),
            });
        }

        let address = start + *index;
        *index += 1;
        Ok(address)
    }

    pub fn get_available_global_address(&mut self, var_type: VarType) -> Result<i64, MemoryError> {
        match var_type {
            VarType::Entero => Self::next_address(
                GLOBAL_INT_START,
                &mut self.int_global_index,
                GLOBAL_LOCAL_MEMORY_SIZE,
                "globales enteras",
            ),
            VarType::Flotante => Self::next_address(
                GLOBAL_FLOAT_START,
                &mut self.float_global_index,
                GLOBAL_LOCAL_MEMORY_SIZE,
                "globales flotantes",
            ),
            VarType::Boleano => Err(MemoryError {
                message: "No se permiten boleanos globales".to_string(),
            }),
        }
    }

    pub fn get_available_local_address(&mut self, var_type: VarType) -> Result<i64, MemoryError> {
        match var_type {
            VarType::Entero => Self::next_address(
                LOCAL_INT_START,
                &mut self.int_local_index,
                GLOBAL_LOCAL_MEMORY_SIZE,
                "locales enteras",
            ),
            VarType::Flotante => Self::next_address(
                LOCAL_FLOAT_START,
                &mut self.float_local_index,
                GLOBAL_LOCAL_MEMORY_SIZE,
                "locales flotantes",
            ),
            VarType::Boleano => Err(MemoryError {
                message: "No se permiten boleanos locales".to_string(),
            }),
        }
    }

    pub fn get_available_temp_address(&mut self, var_type: VarType) -> Result<i64, MemoryError> {
        match var_type {
            VarType::Entero => Self::next_address(
                TEMP_INT_START,
                &mut self.int_temp_index,
                TEMP_MEMORY_SIZE,
                "temporales enteras",
            ),
            VarType::Flotante => Self::next_address(
                TEMP_FLOAT_START,
                &mut self.float_temp_index,
                TEMP_MEMORY_SIZE,
                "temporales floatantes",
            ),
            VarType::Boleano => Self::next_address(
                TEMP_BOOL_START,
                &mut self.bool_temp_index,
                TEMP_MEMORY_SIZE,
                "temporales booleanas",
            ),
        }
    }

    pub fn get_constant_address(&mut self, value: QuadValue) -> i64 {
        match value {
            QuadValue::Entero(val) => {
                if let Some(address) = self.int_constants_dir.get(&val) {
                    return *address;
                }

                let address = CONSTANTS_START + self.constants_index;
                self.constants_index += 1;
                self.int_constants_dir.insert(val, address);
                address
            }
            QuadValue::Flotante(val) => {
                let key = val.to_bits();

                if let Some(address) = self.float_constants_dir.get(&key) {
                    return *address;
                }

                let address = CONSTANTS_START + self.constants_index;
                self.constants_index += 1;
                self.float_constants_dir.insert(key, address);
                address
            }
            QuadValue::Boleano(_) => {
                let address = CONSTANTS_START + self.constants_index;
                self.constants_index += 1;
                address
            }
        }
    }

    pub fn get_constants(&self) -> Vec<ConstantJson> {
        let mut constants: Vec<ConstantJson> = self.int_constants_dir
            .iter()
            .map(|(&val, &address)| ConstantJson {
                address,
                value: ConstantValue::Entero(val),
            })
            .chain(
                self.float_constants_dir
                    .iter()
                    .map(|(&bits, &address)| ConstantJson {
                        address,
                        value: ConstantValue::Flotante(f64::from_bits(bits)),
                    })
            )
            .collect();

        constants.sort_by_key(|c| c.address);
        constants
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocates_scoped_addresses_and_deduplicates_constants() {
        let mut mm = MemoryManager::new();

        assert_eq!(mm.get_available_global_address(VarType::Entero).unwrap(), 1000);
        assert_eq!(mm.get_available_global_address(VarType::Entero).unwrap(), 1001);
        assert_eq!(mm.get_available_global_address(VarType::Flotante).unwrap(), 1500);
        assert_eq!(mm.get_available_local_address(VarType::Entero).unwrap(), 2000);
        assert_eq!(mm.get_available_local_address(VarType::Flotante).unwrap(), 2500);
        assert_eq!(mm.get_available_temp_address(VarType::Entero).unwrap(), 3000);
        assert_eq!(mm.get_available_temp_address(VarType::Flotante).unwrap(), 4000);
        assert_eq!(mm.get_available_temp_address(VarType::Boleano).unwrap(), 5000);

        let first = mm.get_constant_address(QuadValue::Entero(2));
        let repeated = mm.get_constant_address(QuadValue::Entero(2));
        let other = mm.get_constant_address(QuadValue::Flotante(2.0));
        let bool_first = mm.get_constant_address(QuadValue::Boleano(true));
        let bool_second = mm.get_constant_address(QuadValue::Boleano(true));

        assert_eq!(first, 6000);
        assert_eq!(repeated, first);
        assert_eq!(other, 6001);
        assert_eq!(bool_first, 6002);
        assert_eq!(bool_second, 6003);
    }
}