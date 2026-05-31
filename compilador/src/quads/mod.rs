pub mod quad_operators;
pub mod quad_value;
pub mod operator_group;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::collections::VecDeque;

pub use quad_operators::QuadOperator;
pub use crate::semantics::var_types::VarType;
pub use crate::memory::MemoryManager;
pub use crate::semantics::type_matching::TypeMatching;
pub use operator_group::OperatorGroup;
pub use crate::quads::quad_value::QuadValue;

pub struct Quad {
    id: i64,
    operator: QuadOperator,
    left_operand: Option<(i64, VarType)>,
    right_operand: Option<(i64, VarType)>,
    result: (i64, VarType),
}

impl Quad {
    pub fn new(id: i64, operator: QuadOperator, left_operand: (i64, VarType), right_operand: (i64, VarType), result: (i64, VarType)) -> Self {
        Self {
            id: id,
            operator: operator,
            left_operand: Some(left_operand),
            right_operand: Some(right_operand),
            result: result,
        }
    }
}

pub struct QuadGenerator {
    operator_stack: Vec<QuadOperator>,
    variable_stack: Vec<(i64, VarType)>,
    jumps_stack: Vec<i64>,
    quad_queue: VecDeque<Quad>,
    counter: i64,
    type_matching: TypeMatching,
}

impl QuadGenerator {
    pub fn new() -> Self {
        Self {
            operator_stack: Vec::new(),
            variable_stack: Vec::new(),
            jumps_stack: Vec::new(),
            quad_queue: VecDeque::new(),
            counter: 0,
            type_matching: TypeMatching::new(),
        }
    }

    pub fn push_id(&mut self, address: i64, var_type: VarType) {
        self.variable_stack.push((address, var_type));
    }

    pub fn push_constante(&mut self, value: QuadValue) {
        let var_type = match value {
            QuadValue::Entero(_) => VarType::Entero,
            QuadValue::Flotante(_) => VarType::Flotante,
            QuadValue::Boleano(_) => VarType::Boleano,
        };

        let address = MemoryManager::with_instance(|memory_manager| {
            memory_manager.get_constant_address(value)
        });

        self.variable_stack.push((address, var_type));
    }

    pub fn push_operator(&mut self, operator: QuadOperator) {
        self.operator_stack.push(operator);
    }

    pub fn equals_quad(&mut self) {
        if let Some((right_addr, right_type)) = self.variable_stack.pop() {
            if let Some((left_addr, left_type)) = self.variable_stack.pop() {
                let quad = Quad {
                    id: self.counter,
                    operator: QuadOperator::Asignacion,
                    left_operand: Some((right_addr, right_type)),
                    right_operand: None,
                    result: (left_addr, left_type),
                };
                self.quad_queue.push_back(quad);
                self.counter += 1;
            }
        }
    }

    pub fn check_operator(&mut self, group: OperatorGroup) {
        if let Some(&top_operator) = self.operator_stack.last() {
            if group.matches(top_operator) {
                let right_operand_opt = self.variable_stack.pop();
                let left_operand_opt = self.variable_stack.pop();
                let operator_opt = self.operator_stack.pop();

                if let (Some((left_addr, left_type)), Some((right_addr, right_type)), Some(operator)) = (left_operand_opt, right_operand_opt, operator_opt) {
                    if let Ok(op) = crate::semantics::operators::Operator::try_from(operator) {
                        if let Some(result_type) = self.type_matching.get(left_type, right_type, op) {
                            let new_address = MemoryManager::with_instance(|memory_manager| {
                                memory_manager.get_available_temp_address(result_type)
                            })
                            .expect("unable to allocate temporary address");
                            self.quad_queue.push_back(Quad::new(
                                self.counter,
                                operator,
                                (left_addr, left_type),
                                (right_addr, right_type),
                                (new_address, result_type),
                            ));
                            self.counter += 1;
                            self.variable_stack.push((new_address, result_type));
                        }
                    }
                }
            }
        }
    }

    pub fn save_results(&mut self) {
        let file = File::create("quads.txt").expect("unable to create quads.txt");
        let mut writer = BufWriter::new(file);

        writeln!(writer, "operator left_address right_address result_address")
            .expect("unable to write quads header");

        for quad in &self.quad_queue {
            let operator = match quad.operator {
                QuadOperator::Mas => "+",
                QuadOperator::Menos => "-",
                QuadOperator::Multiplicar => "*",
                QuadOperator::Dividir => "/",
                QuadOperator::ComparadorIgual => "==",
                QuadOperator::NoIgual => "!=",
                QuadOperator::Mayor => ">",
                QuadOperator::Menor => "<",
                QuadOperator::Asignacion => "=",
                QuadOperator::Imprime => "print",
                QuadOperator::GoTo => "goto",
                QuadOperator::GoToV => "gotov",
                QuadOperator::GoToF => "gotof",
                QuadOperator::OpenParenthesis => "open_paren",
                QuadOperator::CloseParenthesis => "close_paren",
            };

            let left_address = quad.left_operand.as_ref().map(|operand| operand.0).unwrap_or(-1);
            let right_address = quad.right_operand.as_ref().map(|operand| operand.0).unwrap_or(-1);
            let result_address = quad.result.0;

            writeln!(writer, "{} {} {} {}", operator, left_address, right_address, result_address)
                .expect("unable to write quad row");
        }

        writer.flush().expect("unable to flush quads.txt");
    }
    
}