use scanner_rust::ScannerStr;
use std::error::Error;
use std::io::Write;
use std::{env, fs, io, process};

fn run(source: String) -> Result<(), Box<dyn Error>> {
    let scanner = ScannerStr::new(&source);
    let tokens: Vec<&str> = scanner.collect();
    println!("{:?}", tokens);
    Ok(())
}

fn run_file(filepath: &str) -> Result<(), Box<dyn Error>> {
    let content: String = fs::read_to_string(filepath)?.parse().unwrap();
    run(content)?;
    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => {}
            Err(_) => break,
        }
        run(line)?;
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rulox <script>");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(args[1].as_str())?;
    } else {
        run_prompt()?;
    }
    Ok(())
}
