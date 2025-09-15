use std::io::ErrorKind;

// ============================================================================
// CONSTANTS
// ============================================================================

/// Maximum memory size for LC-3 (64K words)
pub const MEMORY_MAX: usize = 1 << 16;

/// Number of registers in the LC-3 architecture
pub const REG_COUNT: usize = 11; // R0-R7, PC, COND, COUNT

/// Starting address for programs
pub const PC_START: u16 = 0x3000;

// ============================================================================
// REGISTERS
// ============================================================================

/// LC-3 Register enumeration
/// R0-R7 are general purpose registers
/// PC is the Program Counter
/// COND is the Condition Code register
/// COUNT is used for array bounds checking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Registers {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,    
    COND,  
    COUNT, 
}

impl Registers {
    /// Get the number of registers
    pub fn count() -> usize {
        REG_COUNT
    }
}

// ============================================================================
// CONDITION FLAGS
// ============================================================================

/// LC-3 Condition Code Flags
/// These flags indicate the result of the last operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Flags {
    POS = 1 << 0, /* P - Positive result */
    ZRO = 1 << 1, /* Z - Zero result */
    NEG = 1 << 2, /* N - Negative result */
}

impl Flags {
    /// Check if a flag is set in a condition code value
    pub fn is_set_in(&self, condition_code: u16) -> bool {
        (condition_code & (*self as u16)) != 0
    }
}

// ============================================================================
// OPCODES
// ============================================================================

/// LC-3 Opcodes enumeration
/// Each opcode represents a different instruction type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Opcodes {
    BR = 0,  /* Branch - Conditional jump based on condition codes */
    ADD,     /* Add - Add two values and store result */
    LD,      /* Load - Load value from memory into register */
    ST,      /* Store - Store register value to memory */
    JSR,     /* Jump to Subroutine - Call a subroutine */
    AND,     /* Bitwise AND - Perform bitwise AND operation */
    LDR,     /* Load Register - Load from memory using base+offset */
    STR,     /* Store Register - Store to memory using base+offset */
    RTI,     /* Return from Interrupt - Return from interrupt handler */
    NOT,     /* Bitwise NOT - Perform bitwise NOT operation */
    LDI,     /* Load Indirect - Load from memory address stored in memory */
    STI,     /* Store Indirect - Store to memory address stored in memory */
    JMP,     /* Jump - Unconditional jump to register address */
    RES,     /* Reserved - Unused opcode */
    LEA,     /* Load Effective Address - Load address into register */
    TRAP,    /* Trap - Execute system call or interrupt */
}

impl Opcodes {
    /// Convert a u16 opcode value to Opcodes enum
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

    /// Convert Opcodes enum to u16 value
    pub fn to_u16(self) -> u16 {
        self as u16
    }

    /// Get string representation of opcode
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

    /// Get description of what the opcode does
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

// ============================================================================
// TRAP VECTORS
// ============================================================================

/// LC-3 Trap Vectors for system calls
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum TrapVectors {
    GETC = 0x20,  /* Get character from keyboard */
    OUT = 0x21,   /* Output character to console */
    PUTS = 0x22,  /* Output string to console */
    IN = 0x23,    /* Get character from keyboard and echo */
    PUTSP = 0x24, /* Output string with packed characters */
    HALT = 0x25,  /* Halt the program */
}

impl TrapVectors {
    /// Convert a u16 trap vector to TrapVectors enum
    pub fn from_u16(vector: u16) -> Option<TrapVectors> {
        match vector {
            0x20 => Some(TrapVectors::GETC),
            0x21 => Some(TrapVectors::OUT),
            0x22 => Some(TrapVectors::PUTS),
            0x23 => Some(TrapVectors::IN),
            0x24 => Some(TrapVectors::PUTSP),
            0x25 => Some(TrapVectors::HALT),
            _ => None,
        }
    }

    /// Convert TrapVectors enum to u16 value
    pub fn to_u16(self) -> u16 {
        self as u16
    }

    /// Get string representation of trap vector
    pub fn to_string(self) -> &'static str {
        match self {
            TrapVectors::GETC => "GETC",
            TrapVectors::OUT => "OUT",
            TrapVectors::PUTS => "PUTS",
            TrapVectors::IN => "IN",
            TrapVectors::PUTSP => "PUTSP",
            TrapVectors::HALT => "HALT",
        }
    }

    /// Get description of what the trap vector does
    pub fn description(self) -> &'static str {
        match self {
            TrapVectors::GETC => "Get character from keyboard (no echo)",
            TrapVectors::OUT => "Output character to console",
            TrapVectors::PUTS => "Output null-terminated string to console",
            TrapVectors::IN => "Get character from keyboard with echo",
            TrapVectors::PUTSP => "Output string with packed characters",
            TrapVectors::HALT => "Halt the program execution",
        }
    }
}

// ============================================================================
// INSTRUCTION EXTRACTION FUNCTIONS
// ============================================================================

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

// ============================================================================
// SIGN EXTENSION FUNCTIONS
// ============================================================================

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

/// Generic sign extension function
/// Sign extends a value with the specified bit count
pub fn sign_extend(value: u16, bit_count: usize) -> u16 {
    if ((value >> (bit_count - 1)) & 1) == 1 {
        value | (0xFFFF << bit_count)
    } else {
        value
    }
}

// ============================================================================
// CONVERSION IMPLEMENTATIONS
// ============================================================================

/// Convert u16 to Registers enum
impl From<u16> for Registers {
    fn from(value: u16) -> Self {
        match value {
            0 => Registers::R0,
            1 => Registers::R1,
            2 => Registers::R2,
            3 => Registers::R3,
            4 => Registers::R4,
            5 => Registers::R5,
            6 => Registers::R6,
            7 => Registers::R7,
            8 => Registers::PC,
            9 => Registers::COND,
            _ => Registers::R0, // Default to R0 for invalid values
        }
    }
}

/// Convert Registers enum to u16
impl From<Registers> for u16 {
    fn from(reg: Registers) -> Self {
        reg as u16
    }
}

// ============================================================================
// ERROR TYPES
// ============================================================================

/// Custom error type for LC-3 operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LC3Error {
    /// Invalid memory address
    InvalidAddress(u16),
    /// Invalid register
    InvalidRegister(u16),
    /// Invalid opcode
    InvalidOpcode(u16),
    /// Invalid trap vector
    InvalidTrapVector(u16),
    /// Memory access out of bounds
    MemoryOutOfBounds,
    /// Register access out of bounds
    RegisterOutOfBounds,
    /// IO error
    IoError(ErrorKind),
    /// Custom error message
    Custom(String),
}

impl std::fmt::Display for LC3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LC3Error::InvalidAddress(addr) => write!(f, "Invalid memory address: 0x{:04X}", addr),
            LC3Error::InvalidRegister(reg) => write!(f, "Invalid register: {}", reg),
            LC3Error::InvalidOpcode(opcode) => write!(f, "Invalid opcode: 0x{:02X}", opcode),
            LC3Error::InvalidTrapVector(vector) => write!(f, "Invalid trap vector: 0x{:02X}", vector),
            LC3Error::MemoryOutOfBounds => write!(f, "Memory access out of bounds"),
            LC3Error::RegisterOutOfBounds => write!(f, "Register access out of bounds"),
            LC3Error::IoError(kind) => write!(f, "IO error: {:?}", kind),
            LC3Error::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for LC3Error {}

impl From<ErrorKind> for LC3Error {
    fn from(kind: ErrorKind) -> Self {
        LC3Error::IoError(kind)
    }
}

impl From<String> for LC3Error {
    fn from(msg: String) -> Self {
        LC3Error::Custom(msg)
    }
}

impl From<&str> for LC3Error {
    fn from(msg: &str) -> Self {
        LC3Error::Custom(msg.to_string())
    }
}
