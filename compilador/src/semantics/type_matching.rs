use crate::semantics::var_types::VarType;
use crate::semantics::operators::Operator;

pub struct TypeMatching {
    table: [[[Option<VarType>; 4]; 2]; 2]
}

impl TypeMatching {
    pub fn new() -> Self {
        let mut table = Self {
            table: [[[None; 4]; 2]; 2],
        };

        table.insert(VarType::Entero, VarType::Entero, Operator::Mas, VarType::Entero);

        table
    }

    fn insert(&mut self, type1: VarType, type2: VarType, operator: Operator, result: VarType) {
        self.table[type1 as usize][type2 as usize][operator as usize] = Some(result);
    }

    pub fn get(&self, type1: VarType, type2: VarType, operator: Operator) -> Option<VarType> {
        self.table[type1 as usize][type2 as usize][operator as usize]
    }
}
