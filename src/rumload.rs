use std::fs::File;
use std::io::{self, Read};

pub fn load_program(filename: &str) -> io::Result<Vec<u32>> {
    let mut file = File::open(filename)?;
    let mut bytes = Vec::new();

    // Read entire file into bytes
    file.read_to_end(&mut bytes)?;

    let mut program = Vec::new();

    // Every instruction is 4 bytes
    for chunk in bytes.chunks_exact(4) {
        let word = u32::from_be_bytes([
            chunk[0],
            chunk[1],
            chunk[2],
            chunk[3],
        ]);

        program.push(word);
    }

    Ok(program)
}