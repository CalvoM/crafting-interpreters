use rulox::Rulox;
use std::{env, error::Error, process};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Rulox::new();
    if args.len() > 2 {
        println!("Usage: rulox <script>");
        process::exit(64);
    } else if args.len() == 2 {
        interpreter.run_file(args[1].as_str())?;
    } else {
        interpreter.run_prompt()?;
    }
    Ok(())
}
