
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Opcodes {
    BR = 0,  /* Branch */
    ADD,     /* Add */
    LD,      /* Load */
    ST,      /* Store */
    JSR,     /* Jump to Subroutine */
    AND,     /* Bitwise AND */
    LDR,     /* Load Register */
    STR,     /* Store Register */
    RTI,     /* Return from Interrupt (unused) */
    NOT,     /* Bitwise NOT */
    LDI,     /* Load Indirect */
    STI,     /* Store Indirect */
    JMP,     /* Jump */
    RES,     /* Reserved (unused) */
    LEA,     /* Load Effective Address */
    TRAP,    /* Execute Trap */
}

impl Opcodes {
  
    pub fn from_u16(opcode: u16) -> Option<Opcodes> {
        match opcode {
            0 => Some(Opcodes::BR),
            1 => Some(Opcodes::ADD),
            2 => Some(Opcodes::LD),
            3 => Some(Opcodes::ST),
            4 => Some(Opcodes::JSR),
            5 => Some(Opcodes::AND),
            6 => Some(Opcodes::LDR),
            7 => Some(Opcodes::STR),
            8 => Some(Opcodes::RTI),
            9 => Some(Opcodes::NOT),
            10 => Some(Opcodes::LDI),
            11 => Some(Opcodes::STI),
            12 => Some(Opcodes::JMP),
            13 => Some(Opcodes::RES),
            14 => Some(Opcodes::LEA),
            15 => Some(Opcodes::TRAP),
            _ => None,
        }
    }


    pub fn to_u16(self) -> u16 {
        self as u16
    }

    
    pub fn to_string(self) -> &'static str {
        match self {
            Opcodes::BR => "BR",
            Opcodes::ADD => "ADD",
            Opcodes::LD => "LD",
            Opcodes::ST => "ST",
            Opcodes::JSR => "JSR",
            Opcodes::AND => "AND",
            Opcodes::LDR => "LDR",
            Opcodes::STR => "STR",
            Opcodes::RTI => "RTI",
            Opcodes::NOT => "NOT",
            Opcodes::LDI => "LDI",
            Opcodes::STI => "STI",
            Opcodes::JMP => "JMP",
            Opcodes::RES => "RES",
            Opcodes::LEA => "LEA",
            Opcodes::TRAP => "TRAP",
        }
    }

    
    pub fn description(self) -> &'static str {
        match self {
            Opcodes::BR => "Branch - Conditional jump based on condition codes",
            Opcodes::ADD => "Add - Add two values and store result",
            Opcodes::LD => "Load - Load value from memory into register",
            Opcodes::ST => "Store - Store register value to memory",
            Opcodes::JSR => "Jump to Subroutine - Call a subroutine",
            Opcodes::AND => "Bitwise AND - Perform bitwise AND operation",
            Opcodes::LDR => "Load Register - Load from memory using base+offset",
            Opcodes::STR => "Store Register - Store to memory using base+offset",
            Opcodes::RTI => "Return from Interrupt - Return from interrupt handler",
            Opcodes::NOT => "Bitwise NOT - Perform bitwise NOT operation",
            Opcodes::LDI => "Load Indirect - Load from memory address stored in memory",
            Opcodes::STI => "Store Indirect - Store to memory address stored in memory",
            Opcodes::JMP => "Jump - Unconditional jump to register address",
            Opcodes::RES => "Reserved - Unused opcode",
            Opcodes::LEA => "Load Effective Address - Load address into register",
            Opcodes::TRAP => "Trap - Execute system call or interrupt",
        }
    }
}

/// Extract the opcode from a 16-bit instruction
/// The opcode is stored in the top 4 bits (bits 15-12)
pub fn extract_opcode(instruction: u16) -> u16 {
    instruction >> 12
}

/// Extract the destination register from an instruction
/// The destination register is stored in bits 11-9
pub fn extract_dr(instruction: u16) -> u16 {
    (instruction >> 9) & 0x7
}

/// Extract the first source register from an instruction
/// The first source register is stored in bits 8-6
pub fn extract_sr1(instruction: u16) -> u16 {
    (instruction >> 6) & 0x7
}

/// Extract the second source register from an instruction
/// The second source register is stored in bits 2-0
pub fn extract_sr2(instruction: u16) -> u16 {
    instruction & 0x7
}

/// Extract the immediate mode flag from an instruction
/// The immediate mode flag is stored in bit 5
pub fn extract_imm5_flag(instruction: u16) -> bool {
    (instruction & 0x20) != 0
}

/// Extract the 5-bit immediate value from an instruction
/// The immediate value is stored in bits 4-0
pub fn extract_imm5(instruction: u16) -> u16 {
    instruction & 0x1F
}

/// Extract the 9-bit PC-relative offset from an instruction
/// The offset is stored in bits 8-0
pub fn extract_pc_offset9(instruction: u16) -> u16 {
    instruction & 0x1FF
}

/// Extract the 11-bit PC-relative offset from an instruction
/// The offset is stored in bits 10-0
pub fn extract_pc_offset11(instruction: u16) -> u16 {
    instruction & 0x7FF
}

/// Extract the 6-bit offset from an instruction
/// The offset is stored in bits 5-0
pub fn extract_offset6(instruction: u16) -> u16 {
    instruction & 0x3F
}

/// Extract the trap vector from an instruction
/// The trap vector is stored in bits 7-0
pub fn extract_trap_vector(instruction: u16) -> u16 {
    instruction & 0xFF
}

/// Sign extend a 5-bit value to 16 bits
pub fn sign_extend_imm5(value: u16) -> u16 {
    if (value & 0x10) != 0 {
        value | 0xFFE0
    } else {
        value
    }
}

/// Sign extend a 6-bit value to 16 bits
pub fn sign_extend_offset6(value: u16) -> u16 {
    if (value & 0x20) != 0 {
        value | 0xFFC0
    } else {
        value
    }
}

/// Sign extend a 9-bit value to 16 bits
pub fn sign_extend_pc_offset9(value: u16) -> u16 {
    if (value & 0x100) != 0 {
        value | 0xFE00
    } else {
        value
    }
}

/// Sign extend an 11-bit value to 16 bits
pub fn sign_extend_pc_offset11(value: u16) -> u16 {
    if (value & 0x400) != 0 {
        value | 0xF800
    } else {
        value
    }
}
