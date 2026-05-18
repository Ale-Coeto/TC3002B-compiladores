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