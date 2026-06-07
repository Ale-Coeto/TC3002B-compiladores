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

pub enum QuadOperand {
    Address(i64, VarType),
    Name(String),
}

pub struct Quad {
    id: i64,
    operator: QuadOperator,
    left_operand: Option<QuadOperand>,
    right_operand: Option<QuadOperand>,
    result: Option<QuadOperand>,
}

impl Quad {
    pub fn new(id: i64, operator: QuadOperator, left_operand: Option<QuadOperand>, right_operand: Option<QuadOperand>, result: Option<QuadOperand>) -> Self {
        Self {
            id: id,
            operator: operator,
            left_operand: left_operand,
            right_operand: right_operand,
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
    parameter_counter: i64,
    temp_counter: i64,
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
            parameter_counter: 0,
            temp_counter: 0,
        }
    }

    fn quad_operator_to_string(operator: QuadOperator) -> &'static str {
        match operator {
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
            QuadOperator::Era => "era",
            QuadOperator::Param => "param",
            QuadOperator::GoSub => "gosub",
            QuadOperator::Return => "return",
            QuadOperator::EndFunc => "endfunc",
            QuadOperator::End => "end",
        }
    }

    fn operand_to_string(op: &Option<QuadOperand>) -> String {
        match op {
            Some(QuadOperand::Address(addr, _)) => addr.to_string(),
            Some(QuadOperand::Name(name)) => name.clone(),
            None => "-1".to_string(),
        }
    }

    fn quad_to_string(quad: &Quad) -> String {
        let operator = Self::quad_operator_to_string(quad.operator);
        let left = Self::operand_to_string(&quad.left_operand);
        let right = Self::operand_to_string(&quad.right_operand);
        let result = Self::operand_to_string(&quad.result);

        format!("{} {} {} {}", operator, left, right, result)
    }

    pub fn get_counter(&self) -> i64 {
        self.counter
    }

    pub fn get_temp_count(&mut self) -> i64 {
        let temp_count = self.temp_counter;
        self.temp_counter = 0;
        temp_count
    }

    pub fn push_start(&mut self) {
        self.quad_queue.push_back(Quad::new(
            self.counter,
            QuadOperator::GoTo,
            None,
            None,
            None
        ));
        self.jumps_stack.push(self.counter);
        self.counter += 1;
    }

    pub fn push_main(&mut self) {
        if let Some(pending_jump) = self.jumps_stack.pop() {
            if let Some(q) = self.quad_queue.get_mut(pending_jump as usize) {
                q.result = Some(QuadOperand::Address(self.counter, VarType::Entero));
            }
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
                    left_operand: Some(QuadOperand::Address(left_addr, left_type)),
                    right_operand: None,
                    result: Some(QuadOperand::Address(right_addr, right_type)),
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
                            self.temp_counter += 1;
                            self.quad_queue.push_back(Quad::new(
                                self.counter,
                                operator,
                                Some(QuadOperand::Address(left_addr, left_type)),
                                Some(QuadOperand::Address(right_addr, right_type)),
                                Some(QuadOperand::Address(new_address, result_type)),
                            ));
                            self.counter += 1;
                            self.variable_stack.push((new_address, result_type));
                        }
                    }
                }
            }
        }
    }

    pub fn push_if_start(&mut self) {
        if let Some((top_variable, top_type)) = self.variable_stack.pop() {
            if top_type != VarType::Boleano {
                println!("TYPE Mismatch");
            } else {
                self.quad_queue.push_back(Quad::new(
                    self.counter,
                    QuadOperator::GoToF,
                    Some(QuadOperand::Address(top_variable, top_type)),
                    None,
                    None,
                ));
                self.jumps_stack.push(self.counter);
                self.counter += 1;
            }
        }
    }

    pub fn push_else(&mut self) {
        self.quad_queue.push_back(Quad::new(
            self.counter,
            QuadOperator::GoTo,
            None,
            None,
            None,
        ));

        if let Some(pending_jump) = self.jumps_stack.pop() {
            if let Some(q) = self.quad_queue.get_mut(pending_jump as usize) {
                q.result = Some(QuadOperand::Address(self.counter+1, VarType::Entero));
            }
        }

        self.jumps_stack.push(self.counter);
        self.counter += 1;
    }

    pub fn push_if_end(&mut self) {
        if let Some(pending_jump) = self.jumps_stack.pop() {
            if let Some(q) = self.quad_queue.get_mut(pending_jump as usize) {
                q.result = Some(QuadOperand::Address(self.counter, VarType::Entero));
            }
        }
    }

    pub fn push_while_start(&mut self) {
        self.jumps_stack.push(self.counter);
    }

    pub fn push_while_expression(&mut self) {
        if let Some((top_variable, top_type)) = self.variable_stack.pop() {
            if top_type != VarType::Boleano {
                println!("TYPE Mismatch");
            } else {
                self.quad_queue.push_back(Quad::new(
                    self.counter,
                    QuadOperator::GoToF,
                    Some(QuadOperand::Address(top_variable, top_type)),
                    None,
                    None,
                ));
                self.jumps_stack.push(self.counter);
                self.counter += 1;
            }
        }
    }

    pub fn push_while_end(&mut self) {
        if let (Some(pending_jump), Some(return_id)) = (self.jumps_stack.pop(), self.jumps_stack.pop()) {
            self.quad_queue.push_back(Quad::new(
                self.counter,
                QuadOperator::GoTo,
                None,
                None,
                Some(QuadOperand::Address(return_id, VarType::Entero)),
            ));
            if let Some(q) = self.quad_queue.get_mut(pending_jump as usize) {
                q.result = Some(QuadOperand::Address(self.counter+1, VarType::Entero));
            }
            self.counter += 1;
        }
    }

    pub fn push_func_era(&mut self) {
        self.parameter_counter = 0;
        self.quad_queue.push_back(Quad::new(
            self.counter,
            QuadOperator::Era,
            None,
            None,
            None,
        ));
        self.counter += 1;
    }
    pub fn get_parameter_counter(&self) -> i64 {
        self.parameter_counter
    }

    pub fn parameter_counter_next(&mut self) {
        self.parameter_counter += 1;
    }

    pub fn push_func_arg(&mut self) -> (VarType, i64) {
        if let Some((top_var, top_type)) = self.variable_stack.pop() {
            self.quad_queue.push_back(Quad::new(
                self.counter,
                QuadOperator::Param,
                Some(QuadOperand::Address(top_var, top_type)),
                None,
                Some(QuadOperand::Address(self.parameter_counter, VarType::Entero)),
            ));
            self.counter += 1;
            return (top_type, self.parameter_counter);
        }
        (VarType::Entero, -1)
    }

    pub fn push_func_gosub(&mut self, func_name: String) {
        self.quad_queue.push_back(Quad::new(
            self.counter,
            QuadOperator::GoSub,
            None,
            None,
            Some(QuadOperand::Name(func_name)),
        ));
        self.counter += 1;
    }

    pub fn push_func_end(&mut self) {
        self.quad_queue.push_back(Quad::new(
            self.counter,
            QuadOperator::EndFunc,
            None,
            None,
            None,
        ));
        self.counter += 1;
    }

     pub fn push_end(&mut self) {
        self.quad_queue.push_back(Quad::new(
            self.counter,
            QuadOperator::End,
            None,
            None,
            None,
        ));
        self.counter += 1;
    }

    pub fn get_results(&self) -> Vec<String> {
        self.quad_queue.iter().map(Self::quad_to_string).collect()
    }

    pub fn save_results(&self) {
        let file = File::create("quads.txt").expect("unable to create quads.txt");
        let mut writer = BufWriter::new(file);

        writeln!(writer, "operator left_address right_address result_address")
            .expect("unable to write quads header");

        for line in self.get_results() {
            writeln!(writer, "{}", line).expect("unable to write quad row");
        }

        writer.flush().expect("unable to flush quads.txt");
    }
    
}