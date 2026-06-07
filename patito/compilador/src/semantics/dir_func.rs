use crate::memory::MemoryManager;
use crate::semantics::func_types::FuncType;
use crate::semantics::var_types::VarType;
use crate::semantics::semantic_error::SemanticError;
 

use std::collections::HashMap;

pub struct VarDetails {
    address: i64,
    var_type: VarType,
}

pub struct FuncDetails {
    address: i64,
    func_type: FuncType,
    start_index: i64,
    parameters: Option<Vec<VarType>>,
    memory: (i64, i64, i64),
    var_table: Option<HashMap<String, VarDetails>>
}

pub struct DirFunc {
    dir_func: Option<HashMap<String, FuncDetails>>,
    curr_func: Option<String>,
    call_func: Option<String>,
    curr_vars: Vec<String>,
    param_count: i64,
    local_count: i64,
}

pub struct FuncJson {
    pub name: String,
    pub start_index: i64,
    pub local_count: i64,
    pub temp_count: i64,
}

impl DirFunc {
    pub fn new() -> Self {
        Self {
            dir_func: None,
            curr_func: None,
            call_func: None,
            curr_vars: Vec::new(),
            param_count: 0,
            local_count: 0,
        }
    }

    pub fn create_dir_func(&mut self) {
        self.dir_func = Some(HashMap::new());
    }

    pub fn add_func(&mut self, name: String, func_type: FuncType, start_index: i64) -> Result<(), SemanticError> {
        let dir_func = self.dir_func.as_mut()
            .ok_or(SemanticError { message: "No dir_func created".to_string() })?;
        if dir_func.contains_key(&name) {
            return Err(SemanticError { message: "Nombre de función duplicado".to_string() });
        }
        let address = MemoryManager::with_instance(|memory_manager| {
            let mut var_type = VarType::Entero;
            if func_type == FuncType::Flotante {
                var_type = VarType::Flotante;
            }

            if func_type != FuncType::Program && !name.starts_with("global") {
                memory_manager.get_available_global_address(var_type)
            } else {
                Ok(0)
            }
        })
        .map_err(|error| SemanticError { message: error.message })?;

        dir_func.insert(name.clone(), FuncDetails { address, func_type: func_type, start_index, parameters: None, memory: (0,0,0), var_table: None });
        self.curr_func = Some(name);
        Ok(())
    }

    pub fn add_param(&mut self, var_type: VarType) -> Result<(), SemanticError> {
        let dir_func = self.dir_func.as_mut()
            .ok_or(SemanticError { message: "No dir_func created".to_string() })?;

        let curr_func = self.curr_func.as_ref()
            .ok_or(SemanticError { message: "No current function".to_string() })?;


        let func_row = dir_func.get_mut(curr_func)
            .ok_or(SemanticError { message: "Function not found".to_string() })?;

        if func_row.parameters.is_none() {
            func_row.parameters = Some(Vec::new());
        }

        func_row.parameters.as_mut().unwrap().push(var_type);
        self.param_count += 1;

        Ok(())
    }

    pub fn save_param_count(&mut self) -> Result<(), SemanticError> {
        let dir_func = self.dir_func.as_mut()
            .ok_or(SemanticError { message: "No dir_func created".to_string() })?;

        let curr_func = self.curr_func.as_ref()
            .ok_or(SemanticError { message: "No current function".to_string() })?;

        let func_row = dir_func.get_mut(curr_func)
            .ok_or(SemanticError { message: "Function not found".to_string() })?;

        func_row.memory.0 = self.param_count;
        self.param_count = 0;

        Ok(())
    }

    pub fn save_local_count(&mut self) -> Result<(), SemanticError> {
        let dir_func = self.dir_func.as_mut()
            .ok_or(SemanticError { message: "No dir_func created".to_string() })?;

        let curr_func = self.curr_func.as_ref()
            .ok_or(SemanticError { message: "No current function".to_string() })?;

        let func_row = dir_func.get_mut(curr_func)
            .ok_or(SemanticError { message: "Function not found".to_string() })?;

        func_row.memory.1 = self.local_count;
        self.local_count = 0;

        Ok(())
    }

    pub fn add_temp_count(&mut self, temp_count: i64) -> Result<(), SemanticError> {
        let dir_func = self.dir_func.as_mut()
            .ok_or(SemanticError { message: "No dir_func created".to_string() })?;

        let curr_func = self.curr_func.as_ref()
            .ok_or(SemanticError { message: "No current function".to_string() })?;

        let func_row = dir_func.get_mut(curr_func)
            .ok_or(SemanticError { message: "Function not found".to_string() })?;

        func_row.memory.2 = temp_count;

        Ok(())
    }

    pub fn create_var_table(&mut self) -> Result<(), SemanticError> {
        let dir_func = self.dir_func.as_mut()
            .ok_or(SemanticError { message: "No dir_func created".to_string() })?;

        let curr_func = self.curr_func.as_ref()
            .ok_or(SemanticError { message: "No current function".to_string() })?;
        
        dir_func.entry(curr_func.clone()).and_modify(|f| f.var_table = Some(HashMap::new()));
        Ok(())
    }

    pub fn add_var_name(&mut self, name: String) {
        self.curr_vars.push(name);
        self.local_count += 1;
    }

    pub fn add_func_start_index(&mut self, index: i64) {
        if let Some(dir_func) = self.dir_func.as_mut() {
            if let Some(curr_func) = self.curr_func.as_ref() {
                if let Some(func_row) = dir_func.get_mut(curr_func) {
                    func_row.start_index = index;
                }
            }
        }
    }

    pub fn check_func_id(&mut self, name: &String) -> Result<(), SemanticError> {
        let dir_func = self.dir_func.as_ref()
            .ok_or(SemanticError { message: "No dir_func created".to_string() })?;

        if dir_func.contains_key(name) {
            self.call_func = Some(name.clone());
            Ok(())
        } else {
            Err(SemanticError { message: format!("Función '{}' no encontrada", name) })
        }
    }

    pub fn check_func_arg(&mut self, var_type: VarType, param_num: i64) -> Result<(), SemanticError> {
        let dir_func = self.dir_func.as_ref()
            .ok_or(SemanticError { message: "No dir_func created".to_string() })?;

        let call_func = self.call_func.as_ref()
            .ok_or(SemanticError { message: "No function call in progress".to_string() })?;

        let func_row = dir_func.get(call_func)
            .ok_or(SemanticError { message: format!("Función '{}' no encontrada", call_func) })?;

        let parameters = func_row.parameters.as_ref()
            .ok_or(SemanticError { message: format!("La función '{}' no tiene argumentos", call_func) })?;

        let index = param_num as usize;
        let expected_type = parameters.get(index)
            .ok_or(SemanticError { message: format!("No existe argumento en la posición {}", param_num) })?;

        if expected_type != &var_type {
            return Err(SemanticError { message: format!("Tipo de argumento inválido en la posición {}", param_num) });
        }

        Ok(())
    }

    pub fn check_func_param_count(&self, args_count: i64) -> Result<(), SemanticError> {
        let dir_func = self.dir_func.as_ref()
            .ok_or(SemanticError { message: "No dir_func created".to_string() })?;

        let call_func = self.call_func.as_ref()
            .ok_or(SemanticError { message: "No function call in progress".to_string() })?;

        let func_row = dir_func.get(call_func)
            .ok_or(SemanticError { message: format!("Función '{}' no encontrada", call_func) })?;

        let parameters = func_row.parameters.as_ref()
            .ok_or(SemanticError { message: format!("La función '{}' no tiene argumentos", call_func) })?; 

        if parameters.len() as i64 - 1 != args_count {
            return Err(SemanticError { message: format!("El número de argumentos de la función {} no coincide", call_func) })
        }
        Ok(())
    }

    pub fn get_address(&self) -> Result<i64, SemanticError> {
        let dir_func = self.dir_func.as_ref()
            .ok_or(SemanticError { message: "No dir_func created".to_string() })?;

        let call_func = self.call_func.as_ref()
            .ok_or(SemanticError { message: "No function call in progress".to_string() })?;

        let func_row = dir_func.get(call_func)
            .ok_or(SemanticError { message: format!("Función '{}' no encontrada", call_func) })?;

        Ok(func_row.address)
    }

    pub fn get_func_name(&self) -> Result<String, SemanticError> {
        let call_func = self.call_func.as_ref()
            .ok_or(SemanticError { message: "No function call in progress".to_string() })?;

        Ok(call_func.clone())
    }

    pub fn get_dir_func(&self) -> Vec<FuncJson> {
        let Some(dir_func) = self.dir_func.as_ref() else { return Vec::new() };

        dir_func
            .iter()
            .filter(|(name, _)| !name.starts_with("global-"))
            .map(|(name, details)| FuncJson {
                name: name.clone(),
                start_index: details.start_index,
                local_count: details.memory.1,
                temp_count: details.memory.2,
            })
            .collect()
    }

    pub fn release_var_table(&mut self) -> Result<(), SemanticError> {
        let dir_func = self.dir_func.as_mut()
            .ok_or(SemanticError { message: "No dir_func created".to_string() })?;

        let curr_func = self.curr_func.as_ref()
            .ok_or(SemanticError { message: "No current function".to_string() })?;

        let func_row = dir_func.get_mut(curr_func)
            .ok_or(SemanticError { message: "Function not found".to_string() })?;

        func_row.var_table = None;
        self.curr_vars.clear();

        Ok(())
    }

    pub fn add_vars(&mut self, var_type: VarType) -> Result<(), SemanticError> {
        let dir_func = self.dir_func.as_mut()
            .ok_or(SemanticError { message: "No dir_func created".to_string() })?;

        let curr_func = self.curr_func.clone()
            .ok_or(SemanticError { message: "No current function".to_string() })?;
        
        let func_row = dir_func.get_mut(&curr_func)
            .ok_or(SemanticError { message: "Function not found".to_string() })?;

        let var_table = func_row.var_table.as_mut()
            .ok_or(SemanticError { message: "Var table not created".to_string() })?;

        let use_global_addresses = curr_func.starts_with("global-") || func_row.func_type == FuncType::Program;

        for name in self.curr_vars.drain(..) {
            if var_table.contains_key(&name) {
                println!("VARIABLE DUPLICADA");
                return Err(SemanticError { message: "Duplicate Variable".to_string() });
            }
            let address = MemoryManager::with_instance(|memory_manager| {
                if use_global_addresses {
                    memory_manager.get_available_global_address(var_type)
                } else {
                    memory_manager.get_available_local_address(var_type)
                }
            })
            .map_err(|error| SemanticError { message: error.message })?;
            var_table.insert(name, VarDetails { address, var_type });
        }
        Ok(())
    }

    pub fn get_var_info(&self, name: &String) -> Result<(i64, VarType), SemanticError> {
        let dir_func = self.dir_func.as_ref()
            .ok_or(SemanticError { message: "No dir_func created".to_string() })?;
        let curr_func = self.curr_func.as_ref()
            .ok_or(SemanticError { message: "No current function".to_string() })?;

        let entry = dir_func
            .get(curr_func)
            .and_then(|func_row| func_row.var_table.as_ref())
            .and_then(|var_table| var_table.get(name))
            .or_else(|| {
                dir_func
                    .iter()
                    .find(|(func_name, func_row)| {
                        func_name.starts_with("global-") && func_row.var_table.is_some()
                    })
                    .and_then(|(_, func_row)| func_row.var_table.as_ref())
                    .and_then(|var_table| var_table.get(name))
            })
            .ok_or(SemanticError { message: format!("Variable '{}' not found", name) })?;

        Ok((entry.address, entry.var_type))
    }

    pub fn delete_all(&mut self) -> Result<(), SemanticError> {
        let dir_func = self.dir_func.as_mut()
            .ok_or(SemanticError { message: "No dir_func created".to_string() })?;
        
        dir_func.clear();
        self.curr_func = None;
        self.curr_vars.clear();
        MemoryManager::with_instance(|memory_manager| memory_manager.reset());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard, OnceLock};

    static TEST_GUARD: OnceLock<Mutex<()>> = OnceLock::new();

    fn test_guard() -> MutexGuard<'static, ()> {
        TEST_GUARD
            .get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn reset_memory_manager() {
        MemoryManager::with_instance(|memory_manager| memory_manager.reset());
    }

    fn setup_dir_func_with_program() -> DirFunc {
        reset_memory_manager();
        let mut dir_func = DirFunc::new();
        dir_func.create_dir_func();
        dir_func.add_func("programa_principal".to_string(), FuncType::Program, 0).unwrap();
        dir_func
    }

    mod new_and_create_dir_func {
        use super::*;

        #[test]
        fn test_01_01_new_starts_empty() {
            let _guard = test_guard();
            let dir_func = DirFunc::new();

            assert!(dir_func.dir_func.is_none());
            assert!(dir_func.curr_func.is_none());
            assert!(dir_func.curr_vars.is_empty());
        }

        #[test]
        fn test_01_02_create_dir_func_creates_table() {
            let _guard = test_guard();
            let mut dir_func = DirFunc::new();

            dir_func.create_dir_func();

            assert!(dir_func.dir_func.is_some());
        }
    }

    mod add_func {
        use super::*;

        #[test]
        fn test_02_01_success_sets_current_function() {
            let _guard = test_guard();
            reset_memory_manager();
            let mut dir_func = DirFunc::new();
            dir_func.create_dir_func();

            dir_func
                .add_func("mi_funcion".to_string(), FuncType::Entero, 0)
                .unwrap();

            assert_eq!(dir_func.curr_func.as_deref(), Some("mi_funcion"));
            let func_row = dir_func.dir_func.as_ref().unwrap().get("mi_funcion").unwrap();
            assert_eq!(func_row.address, 1000);
        }

        #[test]
        fn test_02_02_errors_without_dir_func() {
            let _guard = test_guard();
            let mut dir_func = DirFunc::new();

            let error = dir_func
                .add_func("mi_funcion".to_string(), FuncType::Entero, 0)
                .unwrap_err();

            assert_eq!(error, SemanticError { message: "No dir_func created".to_string() });
        }

        #[test]
        fn test_02_03_add_func_start_index_updates_current_function() {
            let _guard = test_guard();
            let mut dir_func = DirFunc::new();
            dir_func.create_dir_func();
            dir_func
                .add_func("mi_funcion".to_string(), FuncType::Entero, 0)
                .unwrap();

            dir_func.add_func_start_index(42);

            let func_row = dir_func.dir_func.as_ref().unwrap().get("mi_funcion").unwrap();
            assert_eq!(func_row.start_index, 42);
        }

        #[test]
        fn test_02_04_release_var_table_clears_table() {
            let _guard = test_guard();
            let mut dir_func = setup_dir_func_with_program();
            dir_func.create_var_table().unwrap();

            dir_func.release_var_table().unwrap();

            let func_row = dir_func.dir_func.as_ref().unwrap().get("programa_principal").unwrap();
            assert!(func_row.var_table.is_none());
            assert!(dir_func.curr_vars.is_empty());
        }

        #[test]
        fn test_02_05_check_func_id_validates_existing_function() {
            let _guard = test_guard();
            let mut dir_func = DirFunc::new();
            dir_func.create_dir_func();
            dir_func.add_func("mi_funcion".to_string(), FuncType::Entero, 0).unwrap();

            assert!(dir_func.check_func_id(&"mi_funcion".to_string()).is_ok());
        }

        #[test]
        fn test_02_06_check_func_id_errors_for_missing_function() {
            let _guard = test_guard();
            let mut dir_func = DirFunc::new();
            dir_func.create_dir_func();

            let error = dir_func.check_func_id(&"mi_funcion".to_string()).unwrap_err();

            assert_eq!(error, SemanticError { message: "Función 'mi_funcion' no encontrada".to_string() });
        }

        #[test]
        fn test_02_07_check_func_arg_validates_existing_argument() {
            let _guard = test_guard();
            let mut dir_func = DirFunc::new();
            dir_func.create_dir_func();
            dir_func.add_func("mi_funcion".to_string(), FuncType::Entero, 0).unwrap();
            dir_func.add_param(VarType::Entero).unwrap();
            dir_func.check_func_id(&"mi_funcion".to_string()).unwrap();

            assert!(dir_func.check_func_arg(VarType::Entero, 0).is_ok());
        }

        #[test]
        fn test_02_08_check_func_arg_errors_on_type_mismatch() {
            let _guard = test_guard();
            let mut dir_func = DirFunc::new();
            dir_func.create_dir_func();
            dir_func.add_func("mi_funcion".to_string(), FuncType::Entero, 0).unwrap();
            dir_func.add_param(VarType::Entero).unwrap();
            dir_func.check_func_id(&"mi_funcion".to_string()).unwrap();

            let error = dir_func.check_func_arg(VarType::Flotante, 0).unwrap_err();

            assert_eq!(error, SemanticError { message: "Tipo de argumento inválido en la posición 0".to_string() });
        }

        #[test]
        fn test_02_09_check_func_arg_errors_when_index_does_not_exist() {
            let _guard = test_guard();
            let mut dir_func = DirFunc::new();
            dir_func.create_dir_func();
            dir_func.add_func("mi_funcion".to_string(), FuncType::Entero, 0).unwrap();
            dir_func.add_param(VarType::Entero).unwrap();
            dir_func.check_func_id(&"mi_funcion".to_string()).unwrap();

            let error = dir_func.check_func_arg(VarType::Entero, 1).unwrap_err();

            assert_eq!(error, SemanticError { message: "No existe argumento en la posición 1".to_string() });
        }
    }

    mod create_var_table {
        use super::*;

        #[test]
        fn test_03_01_success() {
            let _guard = test_guard();
            let mut dir_func = setup_dir_func_with_program();

            dir_func.create_var_table().unwrap();

            let func_row = dir_func.dir_func.as_ref().unwrap().get("programa_principal").unwrap();
            assert!(func_row.var_table.is_some());
        }

        #[test]
        fn test_03_02_errors_without_dir_func() {
            let _guard = test_guard();
            let mut dir_func = DirFunc::new();

            let error = dir_func.create_var_table().unwrap_err();

            assert_eq!(error, SemanticError { message: "No dir_func created".to_string() });
        }

        #[test]
        fn test_03_03_errors_without_current_function() {
            let _guard = test_guard();
            let mut dir_func = DirFunc::new();
            dir_func.create_dir_func();

            let error = dir_func.create_var_table().unwrap_err();

            assert_eq!(error, SemanticError { message: "No current function".to_string() });
        }
    }

    mod add_var_name {
        use super::*;

        #[test]
        fn test_04_01_success_pushes_name() {
            let _guard = test_guard();
            let mut dir_func = DirFunc::new();

            dir_func.add_var_name("x".to_string());

            assert_eq!(dir_func.curr_vars, vec!["x".to_string()]);
        }
    }

    mod add_vars {
        use super::*;

        #[test]
        fn test_05_01_success_inserts_all_and_clears_buffer() {
            let _guard = test_guard();
            let mut dir_func = setup_dir_func_with_program();
            dir_func.create_var_table().unwrap();
            dir_func.add_var_name("x".to_string());
            dir_func.add_var_name("y".to_string());

            dir_func.add_vars(VarType::Flotante).unwrap();

            let var_table = dir_func
                .dir_func
                .as_ref()
                .unwrap()
                .get("programa_principal")
                .unwrap()
                .var_table
                .as_ref()
                .unwrap();

            let vx = var_table.get("x").unwrap();
            let vy = var_table.get("y").unwrap();

            assert_eq!(vx.address, 1500);
            assert_eq!(vx.var_type, VarType::Flotante);

            assert_eq!(vy.address, 1501);
            assert_eq!(vy.var_type, VarType::Flotante);
            assert!(dir_func.curr_vars.is_empty());
        }

        #[test]
        fn test_05_01b_save_local_count_updates_memory_tuple() {
            let _guard = test_guard();
            let mut dir_func = setup_dir_func_with_program();
            dir_func.create_var_table().unwrap();
            dir_func.add_var_name("x".to_string());
            dir_func.add_var_name("y".to_string());

            dir_func.add_vars(VarType::Flotante).unwrap();
            dir_func.save_local_count().unwrap();

            let func_row = dir_func.dir_func.as_ref().unwrap().get("programa_principal").unwrap();
            assert_eq!(func_row.memory.1, 2);
        }

        #[test]
        fn test_05_02_errors_without_var_table() {
            let _guard = test_guard();
            let mut dir_func = setup_dir_func_with_program();
            dir_func.add_var_name("x".to_string());

            let error = dir_func.add_vars(VarType::Entero).unwrap_err();

            assert_eq!(error, SemanticError { message: "Var table not created".to_string() });
        }

        #[test]
        fn test_05_03_errors_on_duplicate() {
            let _guard = test_guard();
            let mut dir_func = setup_dir_func_with_program();
            dir_func.create_var_table().unwrap();
            dir_func.add_var_name("x".to_string());
            dir_func.add_var_name("y".to_string());
            dir_func.add_var_name("x".to_string());

            let error = dir_func.add_vars(VarType::Flotante).unwrap_err();

            assert_eq!(error, SemanticError { message: "Duplicate Variable".to_string() });
        }

        #[test]
        fn test_05_05_add_temp_count_updates_memory_tuple() {
            let _guard = test_guard();
            let mut dir_func = setup_dir_func_with_program();

            dir_func.add_temp_count(7).unwrap();

            let func_row = dir_func.dir_func.as_ref().unwrap().get("programa_principal").unwrap();
            assert_eq!(func_row.memory.2, 7);
        }
    }

    mod delete_all {
        use super::*;

        #[test]
        fn test_06_01_success_clears_everything() {
            let _guard = test_guard();
            let mut dir_func = setup_dir_func_with_program();
            dir_func.create_var_table().unwrap();
            dir_func.add_var_name("x".to_string());

            dir_func.delete_all().unwrap();

            assert!(dir_func.dir_func.as_ref().unwrap().is_empty());
            assert!(dir_func.curr_func.is_none());
            assert!(dir_func.curr_vars.is_empty());
        }

        #[test]
        fn test_06_02_errors_without_dir_func() {
            let _guard = test_guard();
            let mut dir_func = DirFunc::new();

            let error = dir_func.delete_all().unwrap_err();

            assert_eq!(error, SemanticError { message: "No dir_func created".to_string() });
        }
    }
}

