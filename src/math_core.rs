use crate::instruction_process::execute;

pub struct Machine {
    pub registers: [u32; 8],
    pub pc: usize,
    pub segments: Vec<Option<Vec<u32>>>,
    pub free_ids: Vec<u32>,
    pub running: bool,
}

impl Machine {
    pub fn new(program: Vec<u32>) -> Self {
        Machine {
            registers: [0; 8],
            pc: 0,
            segments: vec![Some(program)],
            free_ids: Vec::new(),
            running: true,
        }
    }

    pub fn run(&mut self) {
        while self.running {
            let instruction = self.segments[0]
                .as_ref()
                .expect("segment 0 is unmapped")[self.pc];

            self.pc += 1;

            execute(self, instruction);
        }
    }
}