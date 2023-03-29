#![allow(clippy::pedantic)]

mod code;
mod hackasm;
mod parser;

use std::fs::File;
use std::io::Read;
use std::{env, fs};

use hackasm::Assembler;
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_contents = String::new();
    let mut file = File::open(&args[1]).expect("File not found");
    file.read_to_string(&mut file_contents)
        .expect("Could not read file");
    // let mut p = Parser::new(&file_contents);
    // p.clone().print_lines();
    // for _ in 0..8 {
    //     p.advance();
    //     println!("{:?}", p.instruction_type());
    //     println!("Has more lines: {}", p.has_more_lines());
    //     println!("Current Instruction: {:?}", p.current_instruction);
    //     println!("Symbol: {:?}", p.symbol());
    //     println!("Dest: {:?}", p.dest());
    //     println!("Dest Binary: {}", dest(p.dest()));
    //     println!("Comp: {:?}", p.comp());
    //     println!("Comp Binary: {}", comp(p.comp()));
    //     println!("Jump: {:?}", p.jump());
    //     println!("Jump Binary: {}", jump(p.jump()));
    // }
    let mut a = Assembler::new(&file_contents);
    let to_write = a.generate_binary();
    fs::write(
        format!("{}.hack", &args[1].split('.').nth(0).unwrap()),
        to_write.join("\n"),
    )
    .expect("Unable to write file");
}
