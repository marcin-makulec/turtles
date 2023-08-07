use std::env;
use std::error;
use std::fs;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).ok_or("Usage: turtles <filename>")?;
    let input_text = fs::read_to_string(filename)?;
    
    let numbers: Vec<u64> = input_text
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    Ok(())
}
