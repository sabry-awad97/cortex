use crate::error::{CortexError, Result};
use std::fs;
use std::path::Path;

pub struct Interpreter {
    memory: Vec<u8>,
    pointer: usize,
    code: Vec<char>,
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
        self.code = code.chars().collect();
        self.code_pointer = 0;
    }

    pub fn run(&mut self) -> Result<()> {
        while self.code_pointer < self.code.len() {
            self.execute_instruction()?;
        }
        Ok(())
    }

    fn execute_instruction(&mut self) -> Result<()> {
        match self.code[self.code_pointer] {
            '>' => {
                self.pointer += 1;
                if self.pointer >= self.memory.len() {
                    return Err(CortexError::Runtime("Memory pointer out of bounds".into()));
                }
            }
            '<' => {
                self.pointer = self.pointer.checked_sub(1).ok_or_else(|| {
                    CortexError::Runtime("Memory pointer cannot go below zero".into())
                })?;
            }
            '+' => self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1),
            '-' => self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1),
            '.' => {
                use std::io::Write;
                print!("{}", self.memory[self.pointer] as char);
                std::io::stdout().flush()?;
            }
            ',' => {
                use std::io::Read;
                let mut input = [0];
                std::io::stdin().read_exact(&mut input)?;
                self.memory[self.pointer] = input[0];
            }
            '[' => self.handle_loop_start()?,
            ']' => self.handle_loop_end()?,
            _ => {} // Ignore all other characters
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
                    '[' => depth += 1,
                    ']' => depth -= 1,
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
                    ']' => depth += 1,
                    '[' => depth -= 1,
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
