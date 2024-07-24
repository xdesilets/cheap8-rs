use std::{fs::File, io::Read};

use super::environment::*;

pub struct Core {
    pub memory: [u8; MEMORY_SIZE],
    pub program_counter: u16,
    pub general_purpose_register: [u8; GPR_SIZE],
    pub display: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    pub index_register: u16,
    pub stack: [u16; STACK_SIZE]
}

impl Core {

    pub fn get_memory_at(&self, index: usize) -> u8{
        self.memory[index]
    }

    pub fn get_memory_at_pc(&self) -> u8{
        self.get_memory_at(self.program_counter as usize)
    }

    pub fn get_memory_at_next_pc(&self) -> u8{
        self.get_memory_at((self.program_counter + 1) as usize)
    }

    pub fn clear_display(&mut self) {
        for ele in self.display.iter_mut() {
            *ele = false;
        }
    }

    pub fn load_rom(&mut self, mut file: File){
        self.memory = [0; MEMORY_SIZE];
        self.memory[..80].copy_from_slice(&INITIAL_MEMORY);

        let mut buffer = Vec::new();
        match file.read_to_end(&mut buffer){
            Ok(_) => (),
            Err(e) => panic!("Error loading rom's memory: {e}")
        }

        for index in 0..buffer.len() -1{
            self.memory[PROGRAM_START_POSITION as usize + index] = buffer[index];
        }
    }
}

impl Default for Core {
    fn default() -> Core {
        Core {
            memory: [0; MEMORY_SIZE],
            program_counter : PROGRAM_START_POSITION,
            general_purpose_register: [0; GPR_SIZE],
            display: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            index_register: 0,
            stack: [0; STACK_SIZE]
        }
    }
}
