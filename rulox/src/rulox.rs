use scanner_rust::ScannerStr;
use std::error::Error;
use std::io::Write;
use std::{fs, io, process};

pub struct Rulox {
    had_error: bool,
}

impl Rulox {
    pub fn new() -> Rulox {
        Rulox { had_error: false }
    }
    pub fn run(&mut self, source: &str) -> Result<(), Box<dyn Error>> {
        let scanner = ScannerStr::new(&source);
        let tokens: Vec<&str> = scanner.collect();
        println!("{:?}", tokens);
        Ok(())
    }

    pub fn run_file(&mut self, filepath: &str) -> Result<(), Box<dyn Error>> {
        let content: String = fs::read_to_string(filepath)?.parse().unwrap();
        self.run(content.as_str())?;
        if self.had_error {
            process::exit(65);
        }
        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            print!("> ");
            io::stdout().flush()?;
            let mut line = String::new();
            match io::stdin().read_line(&mut line) {
                Ok(_) => {}
                Err(_) => break,
            }
            self.run(line.as_str())?;
            self.had_error = false;
        }
        Ok(())
    }
    pub fn report(
        &mut self,
        line: usize,
        location: &str,
        message: &str,
    ) -> Result<(), Box<dyn Error>> {
        eprintln!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
        Ok(())
    }
    pub fn error(&mut self, line: usize, message: &str) -> Result<(), Box<dyn Error>> {
        self.report(line, "", message)
    }
}
