use std::env::args;
use std::fmt::Write;
use std::fs::File;
use std::io::{self, Read, Write as W};

struct BitfuckInterpreter<'a> {
    memory: [u8; 99999],
    data_pointer: usize,
    source_code: Vec<&'a str>,
}

impl<'a> BitfuckInterpreter<'a> {
    fn new(source_code: Vec<&'a str>) -> Self {
        BitfuckInterpreter {
            memory: [0u8; 99999],
            data_pointer: 0,
            source_code,
        }
    }

    fn execute(&mut self) {
        let mut i = 0;

        while i < self.source_code.len() {
            match self.source_code[i] {
                "ptp" => self.increment_data_pointer(),
                "ptd" => self.decrement_data_pointer(),
                "inc" => self.increment_value(),
                "dec" => self.decrement_value(),
                "out" => self.write_to_output(),
                "ask" => self.read_from_input(),
                "lps" => {
                    if self.memory[self.data_pointer] == 0 {
                        let mut count = 1;
                        while count > 0 {
                            i += 1;
                            if self.source_code[i] == "lps" {
                                count += 1;
                            } else if self.source_code[i] == "lpe" {
                                count -= 1;
                            }
                        }
                    }
                }
                "lpe" => {
                    if self.memory[self.data_pointer] != 0 {
                        let mut count = 1;
                        while count > 0 {
                            i -= 1;
                            if self.source_code[i] == "lpe" {
                                count += 1;
                            } else if self.source_code[i] == "lps" {
                                count -= 1;
                            }
                        }
                    }
                }
                "deb" => self.debug_handler(),
                u => eprintln!("Unknown keyword: {u}"),
            }
            i += 1;
        }
    }

    fn increment_data_pointer(&mut self) {
        if self.data_pointer < self.memory.len() - 1 {
            self.data_pointer += 1;
        }
    }

    fn decrement_data_pointer(&mut self) {
        if self.data_pointer > 0 {
            self.data_pointer -= 1;
        }
    }

    fn increment_value(&mut self) {
        self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_add(1);
    }

    fn decrement_value(&mut self) {
        self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_sub(1);
    }

    fn write_to_output(&self) {
        let byte = self.memory[self.data_pointer];
        let character = char::from(byte);
        print!("{}", character);
        io::stdout().flush().unwrap();
    }

    fn read_from_input(&mut self) {
        let mut buffer = [0u8; 1];
        io::stdin().read_exact(&mut buffer).unwrap();
        self.memory[self.data_pointer] = buffer[0];
    }

    fn debug_handler(&self) {
        let data_pointer_addr = self.source_code.as_ptr() as usize + self.data_pointer;
        let memory_addr = self.memory.as_ptr() as usize + self.data_pointer;
        let memory_value = self.memory[self.data_pointer];

        let mut debug_output = String::new();
        write!(
            &mut debug_output,
            "\x1b[1;34mdebug: \x1b[0mip \x1b[36m=\x1b[0m \x1b[33m0x{:x}\x1b[0m \x1b[90m(\x1b[0mprogram \x1b[36m+\x1b[0m \x1b[33m0x{:x}\x1b[0m\x1b[90m)\x1b[36m;\x1b[0m dp\x1b[36m\x1b[0m \x1b[36m=\x1b[0m \x1b[33m0x{:x}\x1b[0m \x1b[90m(\x1b[0mmemory \x1b[36m+\x1b[0m \x1b[33m0x{:x}\x1b[0m\x1b[90m)\x1b[0m\x1b[36m;\x1b[0m \x1b[36m*\x1b[0mdp \x1b[36m=\x1b[0m \x1b[33m{}\x1b[0m",
            data_pointer_addr,
            self.data_pointer,
            memory_addr,
            self.data_pointer,
            memory_value
        )
        .unwrap();
        println!("{}", debug_output);
    }
}

fn main() {
    let input_filename = args().nth(1).expect("Usage: bitfuck <source file>");

    let mut source_code = String::new();
    let mut input_file = match File::open(input_filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening the file: {}", err);
            return;
        }
    };
    if let Err(err) = input_file.read_to_string(&mut source_code) {
        eprintln!("Error reading from file: {}", err);
        return;
    }

    let mut tokens: Vec<&str> = Vec::new();

    for line in source_code.lines() {
        if !line.trim_start().starts_with('!') {
            let line_tokens: Vec<&str> =
                line.split('!').next().unwrap().split_whitespace().collect();
            tokens.extend(line_tokens);
        }
    }

    let mut interpreter = BitfuckInterpreter::new(tokens);
    interpreter.execute();
}
