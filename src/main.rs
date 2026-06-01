use std::env;
use std::process;

use rum::rumload::load_program;
use rum::math_core::Machine;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Must have exactly one filename argument
    if args.len() != 2 {
        eprintln!("Usage: rum <program.um>");
        process::exit(1);
    }

    let filename = &args[1];

    // Load UM binary file
    let program = match load_program(filename) {
        Ok(words) => words,
        Err(err) => {
            eprintln!("Failed to load file: {}", err);
            process::exit(1);
        }
    };

    // Create machine and run it
    let mut machine = Machine::new(program);
    machine.run();
}