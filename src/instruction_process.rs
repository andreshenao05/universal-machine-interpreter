use crate::math_core::Machine;

use crate::memory::{
    map_segment,
    unmap_segment,
    segmented_load,
    segmented_store,
};

use std::io::{self, Read, Write};

pub fn execute(machine: &mut Machine, instruction: u32) {
    let opcode = instruction >> 28;

    if opcode == 13 {
        let a = ((instruction >> 25) & 7) as usize;
        let value = instruction & 0x1FFFFFF;
        machine.registers[a] = value;
        return;
    }

    let a = ((instruction >> 6) & 7) as usize;
    let b = ((instruction >> 3) & 7) as usize;
    let c = (instruction & 7) as usize;

    match opcode {
        0 => {
            if machine.registers[c] != 0 {
                machine.registers[a] = machine.registers[b];
            }
        }

        1 => {
            machine.registers[a] =
                segmented_load(machine, machine.registers[b], machine.registers[c]);
        }

        2 => {
            segmented_store(
                machine,
                machine.registers[a],
                machine.registers[b],
                machine.registers[c],
            );
        }

        3 => {
            machine.registers[a] =
                machine.registers[b].wrapping_add(machine.registers[c]);
        }

        4 => {
            machine.registers[a] =
                machine.registers[b].wrapping_mul(machine.registers[c]);
        }

        5 => {
            machine.registers[a] =
                machine.registers[b] / machine.registers[c];
        }

        6 => {
            machine.registers[a] =
                !(machine.registers[b] & machine.registers[c]);
        }

        7 => {
            machine.running = false;
        }

        8 => {
            let id = map_segment(machine, machine.registers[c]);
            machine.registers[b] = id;
        }

        9 => {
            unmap_segment(machine, machine.registers[c]);
        }

        10 => {
            let value = machine.registers[c];

            if value > 255 {
                panic!("output > 255");
            }

            print!("{}", value as u8 as char);
            io::stdout().flush().unwrap();
        }

        11 => {
            let mut buffer = [0u8; 1];

            match io::stdin().read(&mut buffer).unwrap() {
                0 => machine.registers[c] = u32::MAX,
                _ => machine.registers[c] = buffer[0] as u32,
            }
        }

        12 => {
            let segment_id = machine.registers[b];

            if segment_id != 0 {
                let new_zero =
                    machine.segments[segment_id as usize]
                        .as_ref()
                        .expect("load from unmapped segment")
                        .clone();

                machine.segments[0] = Some(new_zero);
            }

            machine.pc = machine.registers[c] as usize;
        }

        _ => panic!("invalid opcode {}", opcode),
    }
}