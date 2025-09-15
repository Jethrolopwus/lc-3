use crate::registers::RegisterFile;
use crate::types::{MEMORY_MAX, LC3Error};


#[derive(Debug)]
pub struct Memory {
   
    locations: [u16; MEMORY_MAX],
}

impl Memory {
    
    pub fn new() -> Self {
        Self {
            locations: [0u16; MEMORY_MAX],
        }
    }

   
    pub fn read(&self, address: u16) -> Option<u16> {
        if address as usize >= MEMORY_MAX {
            return None;
        }
        Some(self.locations[address as usize])
    }

  
    pub fn write(&mut self, address: u16, value: u16) -> Result<(), LC3Error> {
        if address as usize >= MEMORY_MAX {
            return Err(LC3Error::MemoryOutOfBounds);
        }
        self.locations[address as usize] = value;
        Ok(())
    }

    pub fn load_program(&mut self, start_address: u16, program: &[u16]) -> Result<usize, LC3Error> {
        if start_address as usize + program.len() > MEMORY_MAX {
            return Err(LC3Error::MemoryOutOfBounds);
        }

        for (i, &instruction) in program.iter().enumerate() {
            self.write(start_address + i as u16, instruction)?;
        }

        Ok(program.len())
    }

    pub fn fetch_instruction(&self, registers: &mut RegisterFile) -> Option<u16> {
        let pc = registers.get_pc();
        let instruction = self.read(pc)?;
        let _ = registers.increment_pc();
        Some(instruction)
    }

   
    pub fn get_memory_slice(&self, start: usize, len: usize) -> &[u16] {
        let end = (start + len).min(MEMORY_MAX);
        &self.locations[start..end]
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
