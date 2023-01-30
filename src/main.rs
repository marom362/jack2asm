#![allow(non_snake_case)]
mod vm2asm;
mod tokenizer;
mod parser;
mod node;
mod compiler;
use std::fs;
use std::io::{self, BufRead, Write};
use std::fs::File;
use std::path::Path;
use std::env;

// "C:\\Users\\adigi\\cb5781\\ecronot\\nand2tetris\\projects\\10\\Square\\Square.jack"
// "C:\\Users\\adigi\\cb5781\\ecronot\\nand2tetris\\projects\\08\\FunctionCalls\\NestedCall"

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg : &String = &args[1];
    let arg_s : &str = &arg[..];

    let tokenizer_ = tokenizer::JackTokenizer::new(arg_s.to_owned());
    let mut tokens = tokenizer_.Tokens;
    let vm = compiler::compile(parser::parse(&mut tokens));
    println!("parsed successfully!");

    //vm2asm::vm2asm(arg_s);

    
    
    let mut xml_file_name = String::new();
    xml_file_name.push_str(arg_s);
    xml_file_name = String::from(&xml_file_name[0..(xml_file_name.len()-5)]);
    xml_file_name.push_str("2.vm");

    let mut buffer = File::create(xml_file_name).unwrap();
    buffer.write_all(vm.as_bytes()).unwrap();
}