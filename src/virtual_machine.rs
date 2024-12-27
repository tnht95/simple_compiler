use crate::code_generator::OpCode;
use std::collections::HashMap;

pub struct VirtualMachine {
    stack: Vec<i64>,
    variables: HashMap<String, i64>,
    instructions: Vec<OpCode>,
    instruction_pointer: usize,
    call_stack: Vec<usize>,
    stack_frames: Vec<Frame>,
    functions: HashMap<String, usize>,
}

#[derive(Debug)]
struct Frame {
    local_variables: HashMap<String, i64>,
    return_address: usize,
}

impl VirtualMachine {
    pub fn new(instructions: Vec<OpCode>) -> Self {
        Self {
            stack: vec![],
            variables: HashMap::new(),
            instructions,
            instruction_pointer: 0,
            call_stack: vec![],
            stack_frames: vec![],
            functions: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        while self.instruction_pointer < self.instructions.len() {
            self.execute(&self.get_current_opcode().clone());
        }
    }

    fn execute(&mut self, opcode: &OpCode) {
        match opcode {
            OpCode::PUSH(value) => self.stack.push(*value),
            // OpCode::POP => {
            //     self.stack.pop().expect("Stack underflow on POP");
            // }
            OpCode::PRINT => {
                if let Some(value) = self.stack.pop() {
                    println!("{}", value);
                } else {
                    panic!("Stack underflow on PRINT");
                }
            }

            // Arithmetic
            OpCode::ADD => self.binary_operation(|a, b| a + b),
            OpCode::SUB => self.binary_operation(|a, b| a - b),
            OpCode::MUL => self.binary_operation(|a, b| a * b),
            OpCode::DIV => self.binary_operation(|a, b| a / b),

            // Variable operations
            OpCode::STORE(name) => {
                let top_value = self.stack.pop().expect("Stack underflow on STORE");

                // check if currently inside a function
                if !self.stack_frames.is_empty() {
                    // push variables to the function's local variables list
                    self.stack_frames
                        .last_mut()
                        .unwrap()
                        .local_variables
                        .insert(name.clone(), top_value);
                } else {
                    self.variables.insert(name.clone(), top_value);
                }
            }
            OpCode::LOAD(name) => {
                let value = self
                    .get_variable(name)
                    .unwrap_or_else(|| panic!("Undefined variable: {}", name));
                self.stack.push(value);
            }

            OpCode::DECLARE(name) => {
                // skip declare opcode to go to enter opcode
                self.functions
                    .insert(name.clone(), self.instruction_pointer + 1);

                // skip handle function until
                while !matches!(self.instructions[self.instruction_pointer], OpCode::EXIT) {
                    self.next_instruction();
                }
            }

            // Function operations
            OpCode::CALL(name) => {
                let next_instruction = self.instruction_pointer + 1;
                // Locate function and set up a new frame
                let frame = Frame {
                    local_variables: HashMap::new(),
                    return_address: next_instruction,
                };
                self.stack_frames.push(frame);
                println!("Allocate stack frame for function: {:?}", name);
                // Jump to the function's start (implement function mapping logic)
                self.call_stack.push(next_instruction);
                self.instruction_pointer = self.find_function_start(name);
            }
            OpCode::TailCall(name) => {
                // Tail call replaces the current frame
                let frame = self
                    .stack_frames
                    .last_mut()
                    .expect("No frame for tail call");
                frame.local_variables.clear();
                println!("Tail call - reuse stack frame for function: {}", name);
                // Jump to the function's start
                self.instruction_pointer = self.find_function_start(name);
            }
            OpCode::RET => {
                if let Some(frame) = self.stack_frames.pop() {
                    self.instruction_pointer = frame.return_address;
                    // skip jumping to the next instruction
                    return;
                } else {
                    panic!("Return with no active frame");
                }
            }

            OpCode::ENTER => {
                self.stack_frames.last_mut().expect("No frame on ENTER");
            }
            OpCode::EXIT => {}

            // Control Flow operations
            OpCode::JUMP(address) => {
                self.instruction_pointer = *address;
                // skip jumping to the next instruction
                return;
            }
            OpCode::JmpIfFalse(address) => {
                if let Some(condition) = self.stack.pop() {
                    if condition == 0 {
                        self.instruction_pointer = *address;
                        // skip jumping to the next instruction
                        return;
                    }
                } else {
                    panic!("Stack underflow on JmpIfFalse");
                }
            }
            // OpCode::JmpIfTrue(address) => {
            //     if let Some(condition) = self.stack.pop() {
            //         if condition != 0 {
            //             self.instruction_pointer = *address;
            //             // skip jumping to the next instruction
            //             return;
            //         }
            //     } else {
            //         panic!("Stack underflow on JmpIfTrue");
            //     }
            // }

            // Comparison operations
            OpCode::EQUAL => self.binary_operation(|a, b| (a == b) as i64),
            OpCode::NotEqual => self.binary_operation(|a, b| (a != b) as i64),
        }

        self.next_instruction();
    }

    fn binary_operation<F>(&mut self, op: F)
    where
        F: FnOnce(i64, i64) -> i64,
    {
        if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
            self.stack.push(op(a, b));
        } else {
            panic!("Stack underflow on binary operation");
        }
    }

    fn find_function_start(&self, name: &String) -> usize {
        *self
            .functions
            .get(name)
            .unwrap_or_else(|| panic!("Undefined function name: {}", name))
    }

    fn get_variable(&self, name: &str) -> Option<i64> {
        self.variables
            .get(name)
            .or_else(|| self.stack_frames.last()?.local_variables.get(name))
            .copied()
    }

    fn get_current_opcode(&self) -> &OpCode {
        &self.instructions[self.instruction_pointer]
    }

    fn next_instruction(&mut self) {
        self.instruction_pointer += 1;
    }
}
