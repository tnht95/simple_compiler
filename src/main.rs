use std::fs::File;
use std::io::Read;
use crate::lexer::Lexer;
use crate::optimizer::Optimizer;
use crate::parser::Parser;

mod lexer;
mod parser;
mod optimizer;

fn main() {
    let  file = File::open("./src/optimization_cases.txt");
    let mut contents = String::new();
    file.unwrap().read_to_string(&mut contents).unwrap();

    println!("{}", contents);
    println!("{}", contents.len());
    println!("==================RUN LEXICAL ANALYZE PHASE===================");
    let tokens = Lexer::tokenize(&contents);

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
    let optimized_expression =  Optimizer::optimize_ast(ast);
    println!("{:#?}", optimized_expression);




}
