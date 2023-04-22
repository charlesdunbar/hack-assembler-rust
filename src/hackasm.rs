#![allow(dead_code, clippy::pedantic)]

use std::collections::HashMap;

use iota::iota;

use crate::code;
use crate::parser::InstructionType;
use crate::parser::Parser;

const SCREEN: u16 = 16384;
const KEYBOARD: u16 = 24576;

iota! {
    const R0:u16 = iota;
    , R1
    , R2
    , R3
    , R4
    , R5
    , R6
    , R7
    , R8
    , R9
    , R10
    , R11
    , R12
    , R13
    , R14
    , R15
}

iota! {
    const SP:u16 = iota;
    , LCL
    , ARG
    , THIS
    , THAT
}

pub struct Assembler<'a> {
    parser: Parser<'a>,
    symbol_table: HashMap<String, u16>,
    to_fill: HashMap<String, Vec<u16>>,
}

impl<'a> Assembler<'a> {
    pub fn new(file: &'a str) -> Self {
        Assembler {
            parser: Parser::new(file),
            symbol_table: HashMap::from([
                (String::from("R0"), R0),
                (String::from("SP"), SP),
                (String::from("R1"), R1),
                (String::from("LCL"), LCL),
                (String::from("R2"), R2),
                (String::from("ARG"), ARG),
                (String::from("R3"), R3),
                (String::from("THIS"), THIS),
                (String::from("R4"), R4),
                (String::from("THAT"), THAT),
                (String::from("R5"), R5),
                (String::from("R6"), R6),
                (String::from("R7"), R7),
                (String::from("R8"), R8),
                (String::from("R9"), R9),
                (String::from("R10"), R10),
                (String::from("R11"), R11),
                (String::from("R12"), R12),
                (String::from("R13"), R13),
                (String::from("R14"), R14),
                (String::from("R15"), R15),
                (String::from("SCREEN"), SCREEN),
                (String::from("KBD"), KEYBOARD),
            ]),
            to_fill: HashMap::new(),
        }
    }

    pub fn generate_binary(&mut self) -> Vec<String> {
        let mut generated_binary = Vec::new();
        let mut current_ram:u16 = 16;

        while self.parser.has_more_lines() {
            self.parser.advance();
            match self.parser.instruction_type() {
                InstructionType::AINSTRUCTION => {
                    // Find out if we're doing a label or number
                    let address_location = self.parser.symbol().unwrap();
                    match address_location.parse::<u16>() {
                        // Format of @number
                        Ok(num) => {
                            generated_binary.push(format!("{:016b}", num));
                            println!("{:016b}", num);
                        }
                        // Format of @label or @variable
                        Err(_) => {
                            // Can either be something like @i, which could be a variable
                            // and need to be in RAM[16] or higher, or could be something like
                            // @LOOP and need to point to the loop location defined later.
                            // If loop is defined before, we can fill it in now.

                            // Plan is to add all to a new data structure, and at the end
                            // of the file, check every element for a reference in symbol table.
                            // If found, add to the correct vec location and remove from data structure.
                            // Everything left over in data structure should be variables in order.
                            // Can add them to RAM[16] and above.
                            if self.symbol_table.contains_key(&address_location) {
                                // If element already in symbol table, add as normal
                                generated_binary.push(format!(
                                    "{:016b}",
                                    self.symbol_table.get(&address_location).unwrap()
                                ));
                                // println!("{:016b}", self.symbol_table.get(&address_location).unwrap());
                            } else {
                                // Otherwise, add a placeholder in the Vec for the file print, and add
                                // to to_fill to be checked after the loop.

                                // Check if we already have a match in the to_fill hash, append string if so
                                self.to_fill
                                    .entry(address_location)
                                    .or_default()
                                    .push(self.parser.current_line);

                                generated_binary.push(String::from("replace"));
                            }
                        }
                    }
                }
                InstructionType::CINSTRUCTION => {
                    generated_binary.push(format!(
                        "111{}{}{}",
                        code::comp(self.parser.comp()),
                        code::dest(self.parser.dest()),
                        code::jump(self.parser.jump())
                    ));
                    // println!(
                    //     "111{}{}{}",
                    //     code::comp(self.parser.comp()),
                    //     code::dest(self.parser.dest()),
                    //     code::jump(self.parser.jump())
                    // );
                }
                InstructionType::LINSTRUCTION => {
                    self.symbol_table.insert(
                        self.parser.symbol().unwrap(),
                        self.parser.current_line,
                    );
                    // println!(
                    //     "{:016b}",
                    //     self.symbol_table
                    //         .get(&self.parser.symbol().unwrap())
                    //         .unwrap()
                    // );
                }
            }
        }

        // println!("Before backfilling, the symbol_table is {:?}", self.symbol_table);
        // println!("Before backfilling, to_fill contains {:?}", self.to_fill);
        // println!(
        //     "And the generated binary vec looks like {:?}",
        //     generated_binary
        // );

        // Fill in labels and variables
        for instruction in self.to_fill.iter_mut() {
            // If not in symbol table at the end, dealing with a variable. Put them in ram and update generated_binary.
            // Also remove the u16 elements from the vec so the next iteration doesn't do anything with them.
            if !self.symbol_table.contains_key(instruction.0) {
                self.symbol_table
                    .insert(instruction.0.clone(), current_ram);
                instruction.1.retain(|line| {
                    // println!(
                    //     "Trying to set variable generated_binary[{}] to {}",
                    //     (*line as usize - 1),
                    //     format!("{:016b}", current_ram)
                    // );
                    generated_binary[*line as usize - 1] = format!("{:016b}", current_ram);
                    false
                });
                current_ram += 1;
            }
            // println!("After variables, instruction is contains {:?}", instruction);

            // If it's a label, update the generated_binary to the vec element it should have.
            for line in instruction.1.iter() {
                // println!(
                //     "Trying to set label generated_binary[{}] to {}",
                //     (*line as usize) - 1,
                //     format!("{:016b}", self.symbol_table.get(instruction.0).unwrap())
                // );
                generated_binary[(*line as usize) - 1] = format!("{:016b}", self.symbol_table.get(instruction.0).unwrap());
            }
        }
        // println!("At the end, to_fill contains {:?}", self.to_fill);
        // println!(
        //     "And the generated binary vec looks like {:?}",
        //     generated_binary
        // );
        generated_binary
    }
}
