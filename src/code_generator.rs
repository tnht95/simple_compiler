use crate::parser::{
    Block, ComparativeOperator, Condition, Expression, Operator, Program, Statement,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum OpCode {
    PUSH(i64), // Push constant onto stack
    // POP,       // Pop value from stack
    PRINT, // Print

    // Arithmetic
    ADD, // Add top two values on stack
    SUB, // Subtract
    MUL, // Multiply
    DIV, // Divide

    // Variable operations
    STORE(String), // Store top of stack in variable
    LOAD(String),  // Load variable onto stack

    // Function operations
    DECLARE(String),  // Declare a function
    TailCall(String), // Tail call function
    CALL(String),     // Call function with name
    RET,              // Return from function
    ENTER,            // Function prologue
    EXIT,             // Function epilogue

    // Control Flow operations
    JUMP(usize),       // Unconditional jump to instruction index
    JmpIfFalse(usize), // Conditional jump if top of stack is false
    // JmpIfTrue(usize),  // Conditional jump if top of stack is true

    // Comparison operations
    EQUAL,    // Compare top two values for equality
    NotEqual, // Compare top two values for inequality
}
pub struct CodeGenerator {
    opcode_list: Vec<OpCode>,
    label_counter: usize,
    label_positions: HashMap<usize, usize>, // Maps label IDs to bytecode_list index
    unresolved_jumps: Vec<(usize, usize)>, // List of (instruction index, label ID) for back-patching
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            opcode_list: vec![],
            label_counter: 0,
            label_positions: Default::default(),
            unresolved_jumps: vec![],
        }
    }

    pub fn generate(&mut self, program: Program) -> Vec<OpCode> {
        match program {
            Program::Statements(statements) => {
                for statement in statements {
                    self.generate_statement(statement);
                }
            }
        }
        self.resolve_labels();
        self.opcode_list.clone()
    }

    fn generate_statement(&mut self, statement: Statement) {
        match statement {
            Statement::VariableDeclaration { identifier, value } => {
                self.generate_expression(value);
                self.opcode_list.push(OpCode::STORE(identifier));
            }
            Statement::Assignment { identifier, value } => {
                self.generate_expression(value);
                self.opcode_list.push(OpCode::STORE(identifier));
            }
            Statement::FunctionDeclaration {
                name,
                parameters,
                body,
                ..
            } => {
                self.opcode_list.push(OpCode::DECLARE(name));
                self.opcode_list.push(OpCode::ENTER);
                for param in parameters.iter().rev() {
                    self.opcode_list.push(OpCode::STORE(param.name.clone()));
                }

                let is_has_return_statement = body.return_expression.is_some();
                self.generate_block(body);

                if !is_has_return_statement {
                    self.opcode_list.push(OpCode::RET);
                }
                self.opcode_list.push(OpCode::EXIT);
            }
            Statement::FunctionCall(expr) => {
                self.generate_expression(expr);
            }
            Statement::Print(expr) => {
                self.generate_expression(expr);
                self.opcode_list.push(OpCode::PRINT);
            }
            Statement::IfStatement {
                condition,
                then_block,
                else_block,
            } => {
                self.generate_condition(condition);
                let else_label = self.get_new_label();
                let end_label = self.get_new_label();

                // 0 is a placeholder
                self.emit_jump(OpCode::JmpIfFalse(0), else_label);
                // Generate the then block
                self.generate_block(then_block);

                // Unconditional jump to skip the else block
                self.emit_jump(OpCode::JUMP(0), end_label);

                // Mark the start of the else block
                self.set_label_position(else_label);

                // Generate the else block, if it exists
                if let Some(else_block) = else_block {
                    self.generate_block(else_block);
                }

                // Mark the end of the if-else statement
                self.set_label_position(end_label);
            }
        }
    }

    // generate code from block and return a boolean
    // which indicates this block has return or not
    fn generate_block(&mut self, block: Block) {
        for statement in block.statements {
            self.generate_statement(statement);
        }

        if let Some(return_expr) = block.return_expression {
            // if return statement only return function call
            match return_expr {
                Expression::FunctionCall { name, arguments } => {
                    for arg in arguments {
                        self.generate_expression(arg);
                    }
                    self.opcode_list.push(OpCode::TailCall(name));
                }
                _ => {
                    self.generate_expression(return_expr);
                }
            }
            self.opcode_list.push(OpCode::RET);
        }
    }

    fn generate_condition(&mut self, condition: Condition) {
        match condition {
            Condition::Comparison {
                left,
                operator,
                right,
            } => {
                self.generate_expression(left);
                self.generate_expression(right);
                self.generate_comparative_operator(operator);
            }
        }
    }

    fn generate_expression(&mut self, expression: Expression) {
        match expression {
            Expression::Integer(value) => {
                self.opcode_list.push(OpCode::PUSH(value));
            }
            Expression::Identifier(name) => {
                self.opcode_list.push(OpCode::LOAD(name));
            }
            Expression::ArithmeticExpression {
                left,
                operator,
                right,
            } => {
                self.generate_expression(*left);
                self.generate_expression(*right);
                self.generate_operator(operator);
            }
            Expression::FunctionCall { name, arguments } => {
                for arg in arguments {
                    self.generate_expression(arg);
                }
                self.opcode_list.push(OpCode::CALL(name));
            }
        }
    }

    fn generate_operator(&mut self, operator: Operator) {
        let opcode = match operator {
            Operator::Add => OpCode::ADD,
            Operator::Subtract => OpCode::SUB,
            Operator::Multiply => OpCode::MUL,
            Operator::Divide => OpCode::DIV,
        };
        self.opcode_list.push(opcode);
    }

    fn generate_comparative_operator(&mut self, operator: ComparativeOperator) {
        let opcode = match operator {
            ComparativeOperator::Equal => OpCode::EQUAL,
            ComparativeOperator::NotEqual => OpCode::NotEqual,
        };
        self.opcode_list.push(opcode);
    }

    fn get_new_label(&mut self) -> usize {
        let label = self.label_counter;
        self.label_counter += 1;
        label
    }

    fn set_label_position(&mut self, label: usize) {
        let position = self.opcode_list.len();
        self.label_positions.insert(label, position);
    }

    fn emit_jump(&mut self, opcode: OpCode, label: usize) {
        let position = self.opcode_list.len();
        self.opcode_list.push(opcode); // Placeholder opcode with unresolved label
        self.unresolved_jumps.push((label, position));
    }

    fn resolve_labels(&mut self) {
        for (label, index) in &self.unresolved_jumps {
            if let Some(&position) = self.label_positions.get(label) {
                if let Some(opcode) = self.opcode_list.get_mut(*index) {
                    match opcode {
                        OpCode::JUMP(ref mut addr_placeholder)
                        | OpCode::JmpIfFalse(ref mut addr_placeholder) => {
                            *addr_placeholder = position;
                        }
                        _ => panic!("Unexpected opcode for label resolution"),
                    }
                }
            } else {
                panic!("Unresolved label: {}", label);
            }
        }
        self.unresolved_jumps.clear();
    }
}
