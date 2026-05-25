use crate::semantics::var_types::{VarType, VAR_TYPE_COUNT};
use crate::semantics::operators::{Operator, OP_COUNT};

pub struct TypeMatching {
    table: [[[Option<VarType>; OP_COUNT]; VAR_TYPE_COUNT]; VAR_TYPE_COUNT]
}

impl TypeMatching {
    pub fn new() -> Self {
        let mut table = Self {
            table: [[[None; OP_COUNT]; VAR_TYPE_COUNT]; VAR_TYPE_COUNT],
        };

        let integer = VarType::Entero;
        let float = VarType::Flotante;
        let boolean = VarType::Boleano;

        for operator in [Operator::Mas, Operator::Menos, Operator::Multiplicar, Operator::Dividir] {
            table.insert(integer, integer, operator, integer);
            table.insert(integer, float, operator, float);
            table.insert(float, integer, operator, float);
            table.insert(float, float, operator, float);
        }

        for operator in [
            Operator::ComparadorIgual,
            Operator::NoIgual,
            Operator::Mayor,
            Operator::Menor,
        ] {
            table.insert(integer, integer, operator, boolean);
            table.insert(integer, float, operator, boolean);
            table.insert(float, integer, operator, boolean);
            table.insert(float, float, operator, boolean);
        }

        table
    }

    fn insert(&mut self, type1: VarType, type2: VarType, operator: Operator, result: VarType) {
        self.table[type1 as usize][type2 as usize][operator as usize] = Some(result);
    }

    pub fn get(&self, type1: VarType, type2: VarType, operator: Operator) -> Option<VarType> {
        self.table[type1 as usize][type2 as usize][operator as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_result(
        table: &TypeMatching,
        left: VarType,
        right: VarType,
        operator: Operator,
        expected: VarType,
    ) {
        assert_eq!(table.get(left, right, operator), Some(expected));
    }

    #[test]
    fn arithmetic_int_int_returns_int() {
        let table = TypeMatching::new();

        for operator in [Operator::Mas, Operator::Menos, Operator::Multiplicar, Operator::Dividir] {
            assert_result(&table, VarType::Entero, VarType::Entero, operator, VarType::Entero);
        }
    }

    #[test]
    fn arithmetic_mixed_and_float_float_returns_float() {
        let table = TypeMatching::new();

        for operator in [Operator::Mas, Operator::Menos, Operator::Multiplicar, Operator::Dividir] {
            assert_result(&table, VarType::Entero, VarType::Flotante, operator, VarType::Flotante);
            assert_result(&table, VarType::Flotante, VarType::Entero, operator, VarType::Flotante);
            assert_result(&table, VarType::Flotante, VarType::Flotante, operator, VarType::Flotante);
        }
    }

    #[test]
    fn comparison_int_int_returns_boolean() {
        let table = TypeMatching::new();

        for operator in [
            Operator::ComparadorIgual,
            Operator::NoIgual,
            Operator::Mayor,
            Operator::Menor,
        ] {
            assert_result(&table, VarType::Entero, VarType::Entero, operator, VarType::Boleano);
        }
    }

    #[test]
    fn comparison_all_type_combinations_return_boolean() {
        let table = TypeMatching::new();

        for operator in [
            Operator::ComparadorIgual,
            Operator::NoIgual,
            Operator::Mayor,
            Operator::Menor,
        ] {
            assert_result(&table, VarType::Entero, VarType::Flotante, operator, VarType::Boleano);
            assert_result(&table, VarType::Flotante, VarType::Entero, operator, VarType::Boleano);
            assert_result(&table, VarType::Flotante, VarType::Flotante, operator, VarType::Boleano);
        }
    }
}
