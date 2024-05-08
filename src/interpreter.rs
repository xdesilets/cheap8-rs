use std::usize;
use self::{core::Core, environment::*};

mod environment;
pub mod core;

pub struct Interpreter {
    pub core: Core,
}

impl Interpreter {
    pub fn new(core: Core) -> Self { Self { core } }

    pub fn fetch(&mut self) -> u16 {
        let instructions: [u8; 2] = [
            self.core.get_memory_at_pc(),
            self.core.get_memory_at_next_pc()
        ]; 

        let instruction: u16 = ((instructions[0] as u16) << 8) | instructions[1] as u16;
        self.core.program_counter += 2;

        return instruction;
    }

    pub fn execute(&mut self, instruction: u16){

        let core: &mut Core = &mut self.core;

        let x: u16 = (instruction & 0x0F00) >> 8; //Second nibble
        let y: u16 = (instruction & 0x0F00) >> 4; //Third nibble
        let n: u16 = (instruction & 0xF000) >> 12; //Fourth nibble
        let nn: u8 = ((instruction & 0xFF00) >> 8) as u8;//Second byte
        let nnn: u16 = (instruction & 0x0FFF) >> 4; //The second, third and fourth nibbles

        match instruction & 0xF000{

            0x0000 => { // Handle 0x0000 instruction

                match n {

                    0x0000 => { //Handle 0x00E0 - CLS
                        core.clear_display();
                    }

                    _ => panic!("Unknown instruction: 0x{:X}", instruction) 
                }
            }

            0x1000 => { // Handle 0x1000 instruction - 1NNN - JUMP NNN
                core.program_counter = nnn as i32;
            }

            0x6000 => {  // Handle 0x6000 instruction - 6XNN - set register VX
                core.general_purpose_register[x as usize] = nn;
            }

            0x7000 => { // Handle 0x7000 instruction - 7XNN - set register VXn
                core.general_purpose_register[x as usize] = core.general_purpose_register[x as usize].wrapping_add(nn);
            }

            0xA000 => { // Handle 0xA000 instruction - ANNN - set index register I
                core.index_register = nnn;
            }

            0xD000 => { // Handle 0xD000 instruction - DXYN - display/draw
                let x_pos: u8 = core.general_purpose_register[x as usize] & 63;
                let y_pos: u8 = core.general_purpose_register[y as usize] & 31;

                core.general_purpose_register[0xF] = 0; //We begin by setting gp a adress 0xF to 0
                
                for sprite_row in 0..n{
                    let sprite_data = core.memory[(core.index_register + sprite_row) as usize];

                    for sprite_bit in 0..8{

                        if x_pos as u16 + sprite_bit >= DISPLAY_WIDTH as u16 || y_pos as u16 + sprite_row >= DISPLAY_HEIGHT as u16{ 
                            continue;
                        }

                        let idx: usize = ((x_pos as u16 + sprite_bit) + ((y_pos as u16 + sprite_row) * DISPLAY_WIDTH as u16)) as usize; 
                        let screen_pixel = core.display[idx];
                        let sprite_pixel = (sprite_data & (1 << 7 - sprite_bit)) != 0;
                        
                        if sprite_pixel & screen_pixel{
                            core.general_purpose_register[0xF] = 1;
                            core.display[idx] = false;
                        }
                    }
                }

            }

            _ => panic!("Unknown instruction: 0x{:X}", instruction) 
        } 
    }

}
