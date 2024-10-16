use crate::error::{CortexError, Result};
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Instruction {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    Output,
    Input,
    LoopStart,
    LoopEnd,
}

impl From<char> for Instruction {
    fn from(ch: char) -> Self {
        match ch {
            '>' => Self::IncrementPointer,
            '<' => Self::DecrementPointer,
            '+' => Self::IncrementValue,
            '-' => Self::DecrementValue,
            '.' => Self::Output,
            ',' => Self::Input,
            '[' => Self::LoopStart,
            ']' => Self::LoopEnd,
            _ => panic!("Invalid character"),
        }
    }
}

pub struct Interpreter {
    memory: Vec<u8>,
    pointer: usize,
    code: Vec<Instruction>,
    code_pointer: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            memory: vec![0; 30000],
            pointer: 0,
            code: Vec::new(),
            code_pointer: 0,
        }
    }

    pub fn load_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let content = fs::read_to_string(path)?;
        self.load_code(&content);
        Ok(())
    }

    pub fn load_code(&mut self, code: &str) {
        self.code = code.chars().map(|c| c.into()).collect();
        self.code_pointer = 0;
    }

    pub fn run(&mut self) -> Result<()> {
        self.check_syntax()?;

        while self.code_pointer < self.code.len() {
            self.execute_instruction()?;
        }
        Ok(())
    }

    fn execute_instruction(&mut self) -> Result<()> {
        match self.code[self.code_pointer] {
            Instruction::IncrementPointer => {
                self.pointer += 1;
                if self.pointer >= self.memory.len() {
                    return Err(CortexError::Runtime("Memory pointer out of bounds".into()));
                }
            }
            Instruction::DecrementPointer => {
                self.pointer = self.pointer.checked_sub(1).ok_or_else(|| {
                    CortexError::Runtime("Memory pointer cannot go below zero".into())
                })?;
            }
            Instruction::IncrementValue => {
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1)
            }
            Instruction::DecrementValue => {
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1)
            }
            Instruction::Output => {
                use std::io::Write;
                print!("{}", self.memory[self.pointer] as char);
                std::io::stdout().flush()?;
            }
            Instruction::Input => {
                use std::io::Read;
                let mut input = [0];
                std::io::stdin().read_exact(&mut input)?;
                self.memory[self.pointer] = input[0];
            }
            Instruction::LoopStart => self.handle_loop_start()?,
            Instruction::LoopEnd => self.handle_loop_end()?,
        }
        self.code_pointer += 1;
        Ok(())
    }

    fn handle_loop_start(&mut self) -> Result<()> {
        if self.memory[self.pointer] == 0 {
            let mut depth = 1;
            while depth > 0 {
                self.code_pointer += 1;
                if self.code_pointer >= self.code.len() {
                    return Err(CortexError::Syntax("Unmatched '[' bracket".into()));
                }
                match self.code[self.code_pointer] {
                    Instruction::LoopStart => depth += 1,
                    Instruction::LoopEnd => depth -= 1,
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn handle_loop_end(&mut self) -> Result<()> {
        if self.memory[self.pointer] != 0 {
            let mut depth = 1;
            while depth > 0 {
                if self.code_pointer == 0 {
                    return Err(CortexError::Syntax("Unmatched ']' bracket".into()));
                }
                self.code_pointer -= 1;
                match self.code[self.code_pointer] {
                    Instruction::LoopEnd => depth += 1,
                    Instruction::LoopStart => depth -= 1,
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn check_syntax(&self) -> Result<()> {
        let mut bracket_stack = Vec::new();

        for (index, ref ch) in self.code.iter().enumerate() {
            match ch {
                Instruction::LoopStart => bracket_stack.push(index),
                Instruction::LoopEnd => {
                    if bracket_stack.pop().is_none() {
                        return Err(CortexError::Syntax(format!(
                            "Unmatched ']' at position {}",
                            index
                        )));
                    }
                }
                _ => {}
            }
        }

        if let Some(position) = bracket_stack.pop() {
            return Err(CortexError::Syntax(format!(
                "Unmatched '[' at position {}",
                position
            )));
        }

        Ok(())
    }
}
