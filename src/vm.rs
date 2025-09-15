use crate::registers::RegisterFile;
use crate::memory::Memory;
use crate::instructions::{InstructionExecutor, ExecutionResult};
use crate::types::Registers;


#[derive(Debug)]
pub struct LC3VM {
    
    pub registers: RegisterFile,
 
    pub memory: Memory,
   
    pub running: bool,
   
    pub instruction_count: u64,
}

impl LC3VM {
   
    pub fn new() -> Self {
        Self {
            registers: RegisterFile::new(),
            memory: Memory::new(),
            running: false,
            instruction_count: 0,
        }
    }

    pub fn initialize(&mut self, start_address: u16, program: &[u16]) -> Result<(), String> {
        
        self.registers.set_pc(start_address)
            .map_err(|_| "Failed to set program counter".to_string())?;

       
        self.memory.load_program(start_address, program)
            .map_err(|_| "Failed to load program".to_string())?;

        
        self.registers.update_condition_code(0)
            .map_err(|_| "Failed to initialize condition code".to_string())?;

        self.running = true;
        self.instruction_count = 0;

        Ok(())
    }

  
    pub fn step(&mut self) -> Result<ExecutionResult, String> {
        if !self.running {
            return Ok(ExecutionResult::Halt);
        }

      
        let instruction = self.memory.fetch_instruction(&mut self.registers)
            .ok_or("Failed to fetch instruction".to_string())?;

       
        let result = InstructionExecutor::execute_instruction(
            instruction,
            &mut self.memory,
            &mut self.registers,
        );

        self.instruction_count += 1;

      
        match result {
            ExecutionResult::Halt => {
                self.running = false;
            }
            ExecutionResult::Error(ref msg) => {
                self.running = false;
                return Err(msg.clone());
            }
            ExecutionResult::Continue => {
               
            }
        }

        Ok(result)
    }

  
    pub fn run(&mut self) -> Result<(), String> {
        while self.running {
            self.step()?;
        }
        Ok(())
    }

  
    pub fn run_for(&mut self, max_instructions: u64) -> Result<(), String> {
        let start_count = self.instruction_count;
        
        while self.running && (self.instruction_count - start_count) < max_instructions {
            self.step()?;
        }
        
        Ok(())
    }

  
    pub fn get_pc(&self) -> u16 {
        self.registers.get_pc()
    }

   
    pub fn get_register(&self, reg: Registers) -> Option<u16> {
        self.registers.read(reg)
    }

   
    pub fn set_register(&mut self, reg: Registers, value: u16) -> Result<(), String> {
        self.registers.write(reg, value)
            .map_err(|e| format!("Failed to write to register: {}", e))
    }

    
    pub fn read_memory(&self, address: u16) -> Option<u16> {
        self.memory.read(address)
    }

  
    pub fn write_memory(&mut self, address: u16, value: u16) -> Result<(), String> {
        self.memory.write(address, value)
            .map_err(|_| "Failed to write to memory".to_string())
    }

   
    pub fn get_instruction_count(&self) -> u64 {
        self.instruction_count
    }

 
    pub fn is_running(&self) -> bool {
        self.running
    }

   
    pub fn halt(&mut self) {
        self.running = false;
    }

   
    pub fn reset(&mut self) {
        self.registers = RegisterFile::new();
        self.memory = Memory::new();
        self.running = false;
        self.instruction_count = 0;
    }

   
    pub fn debug_info(&self) -> String {
        format!(
            "LC-3 VM State:\n\
            PC: 0x{:04X}\n\
            R0: 0x{:04X}  R1: 0x{:04X}  R2: 0x{:04X}  R3: 0x{:04X}\n\
            R4: 0x{:04X}  R5: 0x{:04X}  R6: 0x{:04X}  R7: 0x{:04X}\n\
            COND: 0x{:04X}\n\
            Instructions executed: {}\n\
            Running: {}",
            self.get_pc(),
            self.get_register(Registers::R0).unwrap_or(0),
            self.get_register(Registers::R1).unwrap_or(0),
            self.get_register(Registers::R2).unwrap_or(0),
            self.get_register(Registers::R3).unwrap_or(0),
            self.get_register(Registers::R4).unwrap_or(0),
            self.get_register(Registers::R5).unwrap_or(0),
            self.get_register(Registers::R6).unwrap_or(0),
            self.get_register(Registers::R7).unwrap_or(0),
            self.get_register(Registers::COND).unwrap_or(0),
            self.instruction_count,
            self.running
        )
    }
}

impl Default for LC3VM {
    fn default() -> Self {
        Self::new()
    }
}
