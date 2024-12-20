use std::fs::File;
use std::io::Read;
use crate::lexer::tokenize;
use crate::parser::Parser;

mod lexer;
mod parser;
mod optimizer;

fn main() {
    let  file = File::open("./src/code.txt");
    let mut contents = String::new();
    file.unwrap().read_to_string(&mut contents).unwrap();

    println!("{}", contents);
    println!("{}", contents.len());
    println!("==================RUN LEXICAL ANALYZE PHASE===================");
    let tokens = tokenize(&contents);

    let t = tokens.iter();
    for to in t {
        println!("{:?}", to);
    }


    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => println!("{:#?}", ast),
        Err(e) => eprintln!("Error: {}", e),
    }

}
