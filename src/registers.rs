use crate::types::{Registers, Flags, REG_COUNT, LC3Error};

#[derive(Debug)]
pub struct RegisterFile {
 
    locations: [u16; REG_COUNT],
}

impl RegisterFile {
   
    pub fn new() -> Self {
        Self {
            locations: [0u16; REG_COUNT],
        }
    }

    
    
    pub fn read(&self, reg: Registers) -> Option<u16> {
        if reg as usize >= REG_COUNT {
            return None;
        }
        Some(self.locations[reg as usize])
    }

    
    pub fn write(&mut self, reg: Registers, value: u16) -> Result<(), LC3Error> {
        if reg as usize >= REG_COUNT {
            return Err(LC3Error::RegisterOutOfBounds);
        }
        self.locations[reg as usize] = value;
        Ok(())
    }

    
    pub fn update_condition_code(&mut self, value: u16) -> Result<(), LC3Error> {
        let flag = if value == 0 {
            Flags::ZRO
        } else if (value as i16) < 0 {
            Flags::NEG
        } else {
            Flags::POS
        };
        
        self.write(Registers::COND, flag as u16)
    }

    
    pub fn get_pc(&self) -> u16 {
        self.read(Registers::PC).unwrap_or(0)
    }

    pub fn set_pc(&mut self, value: u16) -> Result<(), LC3Error> {
        self.write(Registers::PC, value)
    }

    
    pub fn increment_pc(&mut self) -> Result<(), LC3Error> {
        let current_pc = self.get_pc();
        self.set_pc(current_pc + 1)
    }

    
    pub fn get_condition_code(&self) -> u16 {
        self.read(Registers::COND).unwrap_or(0)
    }

    
    pub fn is_flag_set(&self, flag: Flags) -> bool {
        (self.get_condition_code() & flag as u16) != 0
    }
}

impl Default for RegisterFile {
    fn default() -> Self {
        Self::new()
    }
}
