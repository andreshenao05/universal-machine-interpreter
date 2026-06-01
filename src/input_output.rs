use std::io::{self, Read, Write};

pub fn write_byte(value: u32) {
    let byte = value as u8;
    io::stdout().write_all(&[byte]).unwrap();
    io::stdout().flush().unwrap();
}

pub fn read_byte() -> u32 {
    let mut buffer = [0u8; 1];

    match io::stdin().read_exact(&mut buffer) {
        Ok(_) => buffer[0] as u32,
        Err(_) => !0,
    }
}