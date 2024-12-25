use crate::parser::{
    Block, ComparativeOperator, Condition, Expression, Operator, Program, Statement,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum OpCode {
    PUSH(i64), // Push constant onto stack
    POP,       // Pop value from stack
    PRINT,     // Print

    // Arithmetic
    ADD, // Add top two values on stack
    SUB, // Subtract
    MUL, // Multiply
    DIV, // Divide

    // Variable operations
    STORE(String), // Store top of stack in variable
    LOAD(String),  // Load variable onto stack

    // Function operations
    TailCall(String, usize), // Tail call function
    CALL(String, usize),     // Call function with name and number of arguments
    RET,                     // Return from function
    ENTER(usize),            // Function prologue (number of local variables)
    EXIT,                    // Function epilogue

    // Control Flow operations
    JUMP(usize),       // Unconditional jump to instruction index
    JmpIfFalse(usize), // Conditional jump if top of stack is false
    JmpIfTrue(usize),  // Conditional jump if top of stack is true

    // Comparison operations
    EQUAL,    // Compare top two values for equality
    NotEqual, // Compare top two values for inequality
}
pub struct CodeGenerator {
    bytecode_list: Vec<OpCode>,
    label_counter: usize,
    label_positions: HashMap<usize, usize>, // Maps label IDs to bytecode_list index
    unresolved_jumps: Vec<(usize, usize)>, // List of (instruction index, label ID) for back-patching
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            bytecode_list: vec![],
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
        self.bytecode_list.clone()
    }

    fn generate_statement(&mut self, statement: Statement) {
        match statement {
            Statement::VariableDeclaration { identifier, value } => {
                self.generate_expression(value);
                self.bytecode_list.push(OpCode::STORE(identifier));
            }
            Statement::Assignment { identifier, value } => {
                self.generate_expression(value);
                self.bytecode_list.push(OpCode::STORE(identifier));
            }
            Statement::FunctionDeclaration {
                parameters, body, ..
            } => {
                self.bytecode_list.push(OpCode::ENTER(parameters.len()));
                let is_has_return_statement = body.return_expression.is_some();
                self.generate_block(body);

                if !is_has_return_statement {
                    self.bytecode_list.push(OpCode::EXIT);
                    self.bytecode_list.push(OpCode::RET);
                }
            }
            Statement::FunctionCall(expr) => {
                self.generate_expression(expr);
            }
            Statement::Print(expr) => {
                self.generate_expression(expr);
                self.bytecode_list.push(OpCode::PRINT);
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
                self.generate_block(then_block);
                self.set_label_position(else_label);

                if let Some(else_statements) = else_block {
                    self.generate_block(else_statements);
                    self.set_label_position(end_label);
                }
            }
        }
    }

    // generate code from block and return a boolean
    // which indicates this block has return or not
    fn generate_block(&mut self, block: Block) -> bool {
        let mut has_return = false;
        for statement in block.statements {
            self.generate_statement(statement);
        }

        if let Some(return_expr) = block.return_expression {
            // if return statement only return function call
            match return_expr {
                Expression::FunctionCall { name, arguments } => {
                    let arguments_length = arguments.len();
                    for arg in arguments {
                        self.generate_expression(arg);
                    }
                    self.bytecode_list
                        .push(OpCode::TailCall(name, arguments_length));
                }
                _ => {
                    self.generate_expression(return_expr);
                }
            }
            self.bytecode_list.push(OpCode::EXIT);
            self.bytecode_list.push(OpCode::RET);
        }

        has_return
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
                self.bytecode_list.push(OpCode::PUSH(value));
            }
            Expression::Identifier(name) => {
                self.bytecode_list.push(OpCode::LOAD(name));
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
                let arguments_length = arguments.len();
                for arg in arguments {
                    self.generate_expression(arg);
                }
                self.bytecode_list
                    .push(OpCode::CALL(name, arguments_length));
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
        self.bytecode_list.push(opcode);
    }

    fn generate_comparative_operator(&mut self, operator: ComparativeOperator) {
        let opcode = match operator {
            ComparativeOperator::Equal => OpCode::EQUAL,
            ComparativeOperator::NotEqual => OpCode::NotEqual,
        };
        self.bytecode_list.push(opcode);
    }

    fn get_new_label(&mut self) -> usize {
        let label = self.label_counter;
        self.label_counter += 1;
        label
    }

    fn set_label_position(&mut self, label: usize) {
        let position = self.bytecode_list.len();
        self.label_positions.insert(label, position);
    }

    fn emit_jump(&mut self, opcode: OpCode, label: usize) {
        let position = self.bytecode_list.len();
        self.bytecode_list.push(opcode); // Placeholder opcode with unresolved label
        self.unresolved_jumps.push((label, position));
    }

    fn resolve_labels(&mut self) {
        for (label, index) in &self.unresolved_jumps {
            if let Some(&position) = self.label_positions.get(label) {
                if let Some(opcode) = self.bytecode_list.get_mut(*index) {
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
