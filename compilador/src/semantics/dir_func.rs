use crate::semantics::func_types::FuncType;
use crate::semantics::var_types::VarType;
use crate::semantics::semantic_error::SemanticError;

use std::collections::HashMap;

pub struct DirFunc {
    dir_func: Option<HashMap<String, (FuncType, Option<HashMap<String, VarType>>)>>,
    curr_func: Option<String>,
    curr_vars: Vec<String>,
}

impl DirFunc {
    pub fn new() -> Self {
        Self {
            dir_func: None,
            curr_func: None,
            curr_vars: Vec::new()
        }
    }

    pub fn create_dir_func(&mut self) {
        self.dir_func = Some(HashMap::new());
    }

    pub fn add_func(&mut self, name: String, func_type: FuncType) -> Result<(), SemanticError> {
        let dir_func = self.dir_func.as_mut()
            .ok_or(SemanticError { message: "No dir_func created".to_string() })?;
        dir_func.insert(name.clone(), (func_type, None));
        self.curr_func = Some(name);
        Ok(())
    }

    pub fn create_var_table(&mut self) -> Result<(), SemanticError> {
        let dir_func = self.dir_func.as_mut()
            .ok_or(SemanticError { message: "No dir_func created".to_string() })?;

        let curr_func = self.curr_func.as_ref()
            .ok_or(SemanticError { message: "No current function".to_string() })?;
        
        dir_func.entry(curr_func.clone()).and_modify(|f| f.1 = Some(HashMap::new()));
        Ok(())
    }

    pub fn add_var_name(&mut self, name: String) {
        self.curr_vars.push(name);
    }

    pub fn add_vars(&mut self, var_type: VarType) -> Result<(), SemanticError> {
        let dir_func = self.dir_func.as_mut()
            .ok_or(SemanticError { message: "No dir_func created".to_string() })?;

        let curr_func = self.curr_func.as_ref()
            .ok_or(SemanticError { message: "No current function".to_string() })?;
        
        let func_row = dir_func.get_mut(curr_func)
            .ok_or(SemanticError { message: "Function not found".to_string() })?;

        let var_table = func_row.1.as_mut()
            .ok_or(SemanticError { message: "Var table not created".to_string() })?;

        for name in self.curr_vars.drain(..) {
            if var_table.contains_key(&name) {
                return Err(SemanticError { message: "Duplicate Variable".to_string() });
            }
            var_table.insert(name, var_type.clone());
        }
        Ok(())
    }

    pub fn delete_all(&mut self) -> Result<(), SemanticError> {
        let dir_func = self.dir_func.as_mut()
            .ok_or(SemanticError { message: "No dir_func created".to_string() })?;
        
        dir_func.clear();
        self.curr_func = None;
        self.curr_vars.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_dir_func_with_program() -> DirFunc {
        let mut dir_func = DirFunc::new();
        dir_func.create_dir_func();
        dir_func.add_func("programa_principal".to_string(), FuncType::Program).unwrap();
        dir_func
    }

    mod new_and_create_dir_func {
        use super::*;

        #[test]
        fn test_01_01_new_starts_empty() {
            let dir_func = DirFunc::new();

            assert!(dir_func.dir_func.is_none());
            assert!(dir_func.curr_func.is_none());
            assert!(dir_func.curr_vars.is_empty());
        }

        #[test]
        fn test_01_02_create_dir_func_creates_table() {
            let mut dir_func = DirFunc::new();

            dir_func.create_dir_func();

            assert!(dir_func.dir_func.is_some());
        }
    }

    mod add_func {
        use super::*;

        #[test]
        fn test_02_01_success_sets_current_function() {
            let mut dir_func = DirFunc::new();
            dir_func.create_dir_func();

            dir_func
                .add_func("mi_funcion".to_string(), FuncType::Entero)
                .unwrap();

            assert_eq!(dir_func.curr_func.as_deref(), Some("mi_funcion"));
            assert!(dir_func.dir_func.as_ref().unwrap().contains_key("mi_funcion"));
        }

        #[test]
        fn test_02_02_errors_without_dir_func() {
            let mut dir_func = DirFunc::new();

            let error = dir_func
                .add_func("mi_funcion".to_string(), FuncType::Entero)
                .unwrap_err();

            assert_eq!(error, SemanticError { message: "No dir_func created".to_string() });
        }
    }

    mod create_var_table {
        use super::*;

        #[test]
        fn test_03_01_success() {
            let mut dir_func = setup_dir_func_with_program();

            dir_func.create_var_table().unwrap();

            let func_row = dir_func.dir_func.as_ref().unwrap().get("programa_principal").unwrap();
            assert!(func_row.1.is_some());
        }

        #[test]
        fn test_03_02_errors_without_dir_func() {
            let mut dir_func = DirFunc::new();

            let error = dir_func.create_var_table().unwrap_err();

            assert_eq!(error, SemanticError { message: "No dir_func created".to_string() });
        }

        #[test]
        fn test_03_03_errors_without_current_function() {
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
            let mut dir_func = DirFunc::new();

            dir_func.add_var_name("x".to_string());

            assert_eq!(dir_func.curr_vars, vec!["x".to_string()]);
        }
    }

    mod add_vars {
        use super::*;

        #[test]
        fn test_05_01_success_inserts_all_and_clears_buffer() {
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
                .1
                .as_ref()
                .unwrap();

            assert_eq!(var_table.get("x"), Some(&VarType::Flotante));
            assert_eq!(var_table.get("y"), Some(&VarType::Flotante));
            assert!(dir_func.curr_vars.is_empty());
        }

        #[test]
        fn test_05_02_errors_without_var_table() {
            let mut dir_func = setup_dir_func_with_program();
            dir_func.add_var_name("x".to_string());

            let error = dir_func.add_vars(VarType::Entero).unwrap_err();

            assert_eq!(error, SemanticError { message: "Var table not created".to_string() });
        }

        #[test]
        fn test_05_03_errors_on_duplicate() {
            let mut dir_func = setup_dir_func_with_program();
            dir_func.create_var_table().unwrap();
            dir_func.add_var_name("x".to_string());
            dir_func.add_var_name("y".to_string());
            dir_func.add_var_name("x".to_string());

            let error = dir_func.add_vars(VarType::Flotante).unwrap_err();

            assert_eq!(error, SemanticError { message: "Duplicate Variable".to_string() });
        }
    }

    mod delete_all {
        use super::*;

        #[test]
        fn test_06_01_success_clears_everything() {
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
            let mut dir_func = DirFunc::new();

            let error = dir_func.delete_all().unwrap_err();

            assert_eq!(error, SemanticError { message: "No dir_func created".to_string() });
        }
    }
}

