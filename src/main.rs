use std::env;
use std::error;
use std::fs;

use turtles::get_critical_number;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).ok_or("Usage: turtles <filename>")?;
    let input_text = fs::read_to_string(filename)?;

    let steps_in_tunnel = input_text
        .lines()
        .filter_map(|line| line.parse::<u128>().ok());

    match get_critical_number(steps_in_tunnel, 100) {
        Some(x) => println!(
            "The tunnel will crumble at number {} on line {}",
            x.step, x.index + 1
        ),
        None => println!("The tunnel will not crumble"),
    }

    Ok(())
}
