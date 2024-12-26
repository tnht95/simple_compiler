use crate::code_generator::CodeGenerator;
use crate::lexer::Lexer;
use crate::optimizer::Optimizer;
use crate::parser::Parser;
use crate::virtual_machine::VirtualMachine;
use std::{env, fs};

mod code_generator;
mod lexer;
mod optimizer;
mod parser;
mod virtual_machine;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    // Read the source file
    let source_code = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error reading file {}: {}", file_path, error);
            std::process::exit(1);
        }
    };

    println!("==================SOURCE CODE===================");

    println!("{}", source_code);
    println!("{}", source_code.len());

    println!("==================RUN LEXICAL ANALYZE PHASE===================");
    let tokens = Lexer::tokenize(&source_code);

    let t = tokens.iter();
    for to in t {
        println!("{:?}", to);
    }

    println!("=================PARSE TOKEN======================");

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => {
            println!("{:#?}", ast);
            ast
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return; // or handle the error case differently
        }
    };

    println!("=================AFTER OPTIMIZE======================");
    let optimized_expression = Optimizer::optimize_ast(ast);
    println!("{:#?}", optimized_expression);

    println!("====================CODE GENERATE=============");
    let mut code_generator = CodeGenerator::new();
    let opcodes = code_generator.generate(optimized_expression);
    let mut a = 0;
    for op in &opcodes {
        println!("{} {:#?}", a, op);
        a = a + 1;
    }

    println!("================VIRTUAL MACHINE====================");
    let mut vm = VirtualMachine::new(opcodes);
    vm.run();
}
