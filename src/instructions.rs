use crate::memory::Memory;
use crate::types::{
    Opcodes, TrapVectors, Flags, Registers,
    extract_dr, extract_imm5, extract_imm5_flag, extract_offset6, extract_pc_offset9,
    extract_pc_offset11, extract_sr1, extract_sr2, extract_trap_vector,
    sign_extend_imm5, sign_extend_offset6, sign_extend_pc_offset9, sign_extend_pc_offset11,
};
use crate::registers::RegisterFile;

#[derive(Debug, PartialEq)]
pub enum ExecutionResult {
    Continue,      
    Halt,          
    Error(String), 
}

pub struct InstructionExecutor;

impl InstructionExecutor {
    /// Sign extend a value with debug prints
    /// Equivalent to: uint16_t sign_extend(uint16_t x, int bit_count)
    pub fn sign_extend(instr: u16, bit_count: usize) -> u16 { 
        println!("=== Sign Extend Debug ===");
        println!("Input: instr = 0x{:04X} ({})", instr, instr);
        println!("Bit count: {}", bit_count);
        
        // Check if sign bit is set: ((x >> (bit_count - 1)) & 1)
        let sign_bit = (instr >> (bit_count - 1)) & 1;
        println!("Sign bit check: (instr >> (bit_count - 1)) & 1 = (0x{:04X} >> {}) & 1 = 0x{:04X} & 1 = {}", 
                 instr, bit_count - 1, instr >> (bit_count - 1), sign_bit);
        
        let mut result = instr;
        
        if sign_bit == 1 {
            println!("Sign bit is set (negative number)");
            println!("Before extension: 0x{:04X} ({})", result, result);
            
            // Perform sign extension: instr |= (0xFFFF << bit_count)
            let mask = 0xFFFF << bit_count;
            println!("Extension mask: 0xFFFF << {} = 0x{:04X}", bit_count, mask);
            
            result |= mask;
            println!("After extension: 0x{:04X} | 0x{:04X} = 0x{:04X} ({})", 
                     instr, mask, result, result as i16);   
        } else {
            println!("Sign bit is clear (positive number) - no extension needed");
        }
        
        println!("Final result: 0x{:04X} ({})", result, result as i16);
        println!("========================");
        
        result
    }
   
    pub fn execute_instruction(
        instruction: u16,
        memory: &mut Memory,
        registers: &mut RegisterFile,
    ) -> ExecutionResult {
        let opcode = (instruction >> 12) as u16;

        match Opcodes::from_u16(opcode) {
            Some(Opcodes::BR) => Self::execute_br(instruction, registers),
            Some(Opcodes::ADD) => Self::execute_add(instruction, registers),
            Some(Opcodes::LD) => Self::execute_ld(instruction, memory, registers),
            Some(Opcodes::ST) => Self::execute_st(instruction, memory, registers),
            Some(Opcodes::JSR) => Self::execute_jsr(instruction, registers),
            Some(Opcodes::AND) => Self::execute_and(instruction, registers),
            Some(Opcodes::LDR) => Self::execute_ldr(instruction, memory, registers),
            Some(Opcodes::STR) => Self::execute_str(instruction, memory, registers),
            Some(Opcodes::RTI) => {
                ExecutionResult::Error("RTI instruction not implemented".to_string())
            }
            Some(Opcodes::NOT) => Self::execute_not(instruction, registers),
            Some(Opcodes::LDI) => Self::execute_ldi(instruction, memory, registers),
            Some(Opcodes::STI) => Self::execute_sti(instruction, memory, registers),
            Some(Opcodes::JMP) => Self::execute_jmp(instruction, registers),
            Some(Opcodes::RES) => ExecutionResult::Error("RES instruction is reserved".to_string()),
            Some(Opcodes::LEA) => Self::execute_lea(instruction, registers),
            Some(Opcodes::TRAP) => Self::execute_trap(instruction, memory, registers),
            None => ExecutionResult::Error(format!("Unknown opcode: {}", opcode)),
        }
    }

   
    fn execute_br(instruction: u16, registers: &mut RegisterFile) -> ExecutionResult {
        let pc_offset9 = extract_pc_offset9(instruction);
        let nzp = (instruction >> 9) & 0x7;

        let should_branch = (nzp & 0x4 != 0 && registers.is_flag_set(Flags::NEG))
            || (nzp & 0x2 != 0 && registers.is_flag_set(Flags::ZRO))
            || (nzp & 0x1 != 0 && registers.is_flag_set(Flags::POS));

        if should_branch {
            let pc = registers.get_pc();
            let offset = sign_extend_pc_offset9(pc_offset9);
            let _ = registers.set_pc(pc + offset);
        }

        ExecutionResult::Continue
    }
    //=== Execute ADD instruction ===
    fn execute_add(instruction: u16, registers: &mut RegisterFile) -> ExecutionResult {
        let dr = extract_dr(instruction);
        let sr1 = extract_sr1(instruction);
        let imm5_flag = extract_imm5_flag(instruction);

        let sr1_value = registers.read(Registers::from(sr1)).unwrap_or(0);
        let result = if imm5_flag {
            let imm5 = extract_imm5(instruction);
            let imm5_value = sign_extend_imm5(imm5);
            sr1_value.wrapping_add(imm5_value)
        } else {
            let sr2 = extract_sr2(instruction);
            let sr2_value = registers.read(Registers::from(sr2)).unwrap_or(0);
            sr1_value.wrapping_add(sr2_value)
        };

        let _ = registers.write(Registers::from(dr), result);
        let _ = registers.update_condition_code(result);

        ExecutionResult::Continue
    }

    
    //=== Load a value from memory into a register ===
    fn execute_ld(
        instruction: u16,
        memory: &Memory,
        registers: &mut RegisterFile,
    ) -> ExecutionResult {
        let dr = extract_dr(instruction);
        let pc_offset9 = extract_pc_offset9(instruction);

        let pc = registers.get_pc();
        let address = pc + sign_extend_pc_offset9(pc_offset9);

        match memory.read(address) {
            Some(value) => {
                let _ = registers.write(Registers::from(dr), value);
                let _ = registers.update_condition_code(value);
                ExecutionResult::Continue
            }
            None => ExecutionResult::Error("Memory read out of bounds".to_string()),
        }
    }

    //=== Store a register value to memory ===
    fn execute_st(
        instruction: u16,
        memory: &mut Memory,
        registers: &mut RegisterFile,
    ) -> ExecutionResult {
        let sr = extract_dr(instruction); 
        let pc_offset9 = extract_pc_offset9(instruction);

        let pc = registers.get_pc();
        let address = pc + sign_extend_pc_offset9(pc_offset9);
        let value = registers.read(Registers::from(sr)).unwrap_or(0);

        match memory.write(address, value) {
            Ok(_) => ExecutionResult::Continue,
            Err(_) => ExecutionResult::Error("Memory write out of bounds".to_string()),
        }
    }

    //=== Save PC and jump to subroutine ====
    fn execute_jsr(instruction: u16, registers: &mut RegisterFile) -> ExecutionResult {
        let pc = registers.get_pc();
        let _ = registers.write(Registers::R7, pc); 

        if (instruction & 0x800) != 0 {
            
            let pc_offset11 = extract_pc_offset11(instruction);
            let offset = sign_extend_pc_offset11(pc_offset11);
            let _ = registers.set_pc(pc + offset);
        } else {
           
            let base_reg = extract_sr1(instruction);
            let base_value = registers.read(Registers::from(base_reg)).unwrap_or(0);
            let _ = registers.set_pc(base_value);
        }

        ExecutionResult::Continue
    }

    
    //==== Perform bitwise AND operation ====
    fn execute_and(instruction: u16, registers: &mut RegisterFile) -> ExecutionResult {
        let dr = extract_dr(instruction);
        let sr1 = extract_sr1(instruction);
        let imm5_flag = extract_imm5_flag(instruction);

        let sr1_value = registers.read(Registers::from(sr1)).unwrap_or(0);
        let result = if imm5_flag {
            let imm5 = extract_imm5(instruction);
            let imm5_value = sign_extend_imm5(imm5);
            sr1_value & imm5_value
        } else {
            let sr2 = extract_sr2(instruction);
            let sr2_value = registers.read(Registers::from(sr2)).unwrap_or(0);
            sr1_value & sr2_value
        };

        let _ = registers.write(Registers::from(dr), result);
        let _ = registers.update_condition_code(result);

        ExecutionResult::Continue
    }

   
   
    fn execute_ldr(
        instruction: u16,
        memory: &Memory,
        registers: &mut RegisterFile,
    ) -> ExecutionResult {
        let dr = extract_dr(instruction);
        let base_reg = extract_sr1(instruction);
        let offset6 = extract_offset6(instruction);

        let base_value = registers.read(Registers::from(base_reg)).unwrap_or(0);
        let offset = sign_extend_offset6(offset6);
        let address = base_value + offset;

        match memory.read(address) {
            Some(value) => {
                let _ = registers.write(Registers::from(dr), value);
                let _ = registers.update_condition_code(value);
                ExecutionResult::Continue
            }
            None => ExecutionResult::Error("Memory read out of bounds".to_string()),
        }
    }

   
    fn execute_str(
        instruction: u16,
        memory: &mut Memory,
        registers: &mut RegisterFile,
    ) -> ExecutionResult {
        let sr = extract_dr(instruction); 
        let base_reg = extract_sr1(instruction);
        let offset6 = extract_offset6(instruction);

        let base_value = registers.read(Registers::from(base_reg)).unwrap_or(0);
        let offset = sign_extend_offset6(offset6);
        let address = base_value + offset;
        let value = registers.read(Registers::from(sr)).unwrap_or(0);

        match memory.write(address, value) {
            Ok(_) => ExecutionResult::Continue,
            Err(_) => ExecutionResult::Error("Memory write out of bounds".to_string()),
        }
    }

   
    //=== Perform bitwise NOT operation ===
    fn execute_not(instruction: u16, registers: &mut RegisterFile) -> ExecutionResult {
        let dr = extract_dr(instruction);
        let sr = extract_sr1(instruction);

        let sr_value = registers.read(Registers::from(sr)).unwrap_or(0);
        let result = !sr_value;

        let _ = registers.write(Registers::from(dr), result);
        let _ = registers.update_condition_code(result);

        ExecutionResult::Continue
    }


    fn execute_ldi(
        instruction: u16,
        memory: &Memory,
        registers: &mut RegisterFile,
    ) -> ExecutionResult {
        let dr = extract_dr(instruction);
        let pc_offset9 = extract_pc_offset9(instruction);

        let pc = registers.get_pc();
        let indirect_address = pc + sign_extend_pc_offset9(pc_offset9);

        match memory.read(indirect_address) {
            Some(direct_address) => match memory.read(direct_address) {
                Some(value) => {
                    let _ = registers.write(Registers::from(dr), value);
                    let _ = registers.update_condition_code(value);
                    ExecutionResult::Continue
                }
                None => ExecutionResult::Error("Indirect memory read out of bounds".to_string()),
            },
            None => ExecutionResult::Error("Memory read out of bounds".to_string()),
        }
    }

    fn execute_sti(
        instruction: u16,
        memory: &mut Memory,
        registers: &mut RegisterFile,
    ) -> ExecutionResult {
        let sr = extract_dr(instruction); 
        let pc_offset9 = extract_pc_offset9(instruction);

        let pc = registers.get_pc();
        let indirect_address = pc + sign_extend_pc_offset9(pc_offset9);
        let value = registers.read(Registers::from(sr)).unwrap_or(0);

        match memory.read(indirect_address) {
            Some(direct_address) => match memory.write(direct_address, value) {
                Ok(_) => ExecutionResult::Continue,
                Err(_) => ExecutionResult::Error("Indirect memory write out of bounds".to_string()),
            },
            None => ExecutionResult::Error("Memory read out of bounds".to_string()),
        }
    }

    
    fn execute_jmp(instruction: u16, registers: &mut RegisterFile) -> ExecutionResult {
        let base_reg = extract_sr1(instruction);
        let base_value = registers.read(Registers::from(base_reg)).unwrap_or(0);
        let _ = registers.set_pc(base_value);

        ExecutionResult::Continue
    }

   
    fn execute_lea(instruction: u16, registers: &mut RegisterFile) -> ExecutionResult {
        let dr = extract_dr(instruction);
        let pc_offset9 = extract_pc_offset9(instruction);

        let pc = registers.get_pc();
        let address = pc + sign_extend_pc_offset9(pc_offset9);

        let _ = registers.write(Registers::from(dr), address);
        let _ = registers.update_condition_code(address);

        ExecutionResult::Continue
    }

    fn execute_trap(
        instruction: u16,
        _memory: &mut Memory,
        _registers: &mut RegisterFile,
    ) -> ExecutionResult {
        let trap_vector = extract_trap_vector(instruction);

        match TrapVectors::from_u16(trap_vector) {
            Some(TrapVectors::GETC) => {
                println!("TRAP: GETC (not implemented)");
                ExecutionResult::Continue
            }
            Some(TrapVectors::OUT) => {
                println!("TRAP: OUT (not implemented)");
                ExecutionResult::Continue
            }
            Some(TrapVectors::PUTS) => {
                println!("TRAP: PUTS (not implemented)");
                ExecutionResult::Continue
            }
            Some(TrapVectors::IN) => {
                println!("TRAP: IN (not implemented)");
                ExecutionResult::Continue
            }
            Some(TrapVectors::PUTSP) => {
                println!("TRAP: PUTSP (not implemented)");
                ExecutionResult::Continue
            }
            Some(TrapVectors::HALT) => {
                println!("TRAP: HALT");
                ExecutionResult::Halt
            }
            None => ExecutionResult::Error(format!("Unknown trap vector: 0x{:02X}", trap_vector)),
        }
    }
}

// From<u16> for Registers is now implemented in types.rs
