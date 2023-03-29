#![allow(clippy::pedantic)]

use std::str::Lines;

#[derive(Clone)]
pub struct Parser<'a> {
    pub current_line: u16,
    source_iterator: Lines<'a>,
    pub current_instruction: Option<&'a str>,
}

#[derive(Debug, PartialEq)]
pub enum InstructionType {
    AINSTRUCTION,
    CINSTRUCTION,
    LINSTRUCTION,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            current_line: 0,
            source_iterator: input.lines(),
            current_instruction: None,
        }
    }

    pub fn print_lines(self) {
        self.source_iterator.enumerate().for_each(|(num, line)| {
            println!("{}: {}", num + 1, line);
        })
    }

    pub fn has_more_lines(&self) -> bool {
        if let Some(_) = self.source_iterator.clone().peekable().peek() {
            true
        } else {
            false
        }
    }

    /// Advance the iterator to the next instruction
    /// which is the first non-blank, non-comment line.
    ///
    /// Updates the current_instruction and current_line fields
    /// while also advancing the source_iterator iteration.
    pub fn advance(&mut self) {
        if !self.has_more_lines() {
            return;
        }

        match self.source_iterator.next() {
            None => {
                self.current_instruction = None;
            }
            Some(line) => {
                match line.chars().next() {
                    None => {
                        // Skip blank lines
                        self.advance();
                    }
                    // Look at first character in the line
                    Some(c) => {
                        //println!("at {}", c);
                        match c {
                            '/' => {
                                if line.chars().next() == Some('/') {
                                    // Do nothing to skip over comments
                                    self.advance();
                                }
                            }
                            _ => {
                                self.current_instruction =
                                    Some(line.split("//").next().unwrap().trim());
                                // println!(
                                //     "{}: {}",
                                //     self.current_line,
                                //     self.current_instruction.unwrap()
                                // );
                                if InstructionType::LINSTRUCTION != self.instruction_type() {
                                    self.current_line += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn instruction_type(&self) -> InstructionType {
        if let Some('@') = self.current_instruction.unwrap().chars().next() {
            return InstructionType::AINSTRUCTION;
        } else if let Some('(') = self.current_instruction.unwrap().chars().next() {
            return InstructionType::LINSTRUCTION;
        }
        InstructionType::CINSTRUCTION
    }

    pub fn symbol(&self) -> Option<String> {
        // Symbol only applies for A or L instructions
        if let InstructionType::CINSTRUCTION = self.instruction_type() {
            return None;
        }
        Some(self.current_instruction?.replace(&['@', '(', ')'][..], ""))
    }

    pub fn dest(&self) -> Option<String> {
        match self.instruction_type() {
            InstructionType::AINSTRUCTION | InstructionType::LINSTRUCTION => None,
            InstructionType::CINSTRUCTION => {
                if let Some(s) = self.current_instruction?.split('=').next() {
                    if s.contains(';') {
                        return None;
                    }
                    return Some(String::from(s));
                } else {
                    None
                }
            }
        }
    }

    pub fn comp(&self) -> Option<String> {
        match self.instruction_type() {
            InstructionType::AINSTRUCTION | InstructionType::LINSTRUCTION => None,
            InstructionType::CINSTRUCTION => {
                if let Some(s) = self.current_instruction?.split('=').nth(1) {
                    if let Some(x) = s.split(';').next() {
                        Some(String::from(x))
                    } else {
                        None
                    }
                } else if let Some(x) = self.current_instruction?.split(';').next() {
                    Some(String::from(x))
                } else {
                    None
                }
            }
        }
    }

    pub fn jump(&self) -> Option<String> {
        match self.instruction_type() {
            InstructionType::AINSTRUCTION | InstructionType::LINSTRUCTION => None,
            InstructionType::CINSTRUCTION => {
                if let Some(s) = self.current_instruction?.split(';').nth(1) {
                    Some(String::from(s))
                } else {
                    None
                }
            }
        }
    }
}
