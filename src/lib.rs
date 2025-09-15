pub mod types;
pub mod registers;
pub mod memory;
pub mod opcodes;
pub mod instructions;
pub mod vm;

// Re-export types for convenience
pub use types::{
    Registers, Flags, Opcodes, TrapVectors, LC3Error,
    MEMORY_MAX, REG_COUNT, PC_START,
    extract_opcode, extract_dr, extract_sr1, extract_sr2,
    extract_imm5_flag, extract_imm5, extract_pc_offset9, extract_pc_offset11,
    extract_offset6, extract_trap_vector,
    sign_extend_imm5, sign_extend_offset6, sign_extend_pc_offset9,
    sign_extend_pc_offset11, sign_extend,
};

// Re-export module-specific types
pub use registers::RegisterFile;
pub use memory::Memory;
pub use instructions::{InstructionExecutor, ExecutionResult};
pub use vm::LC3VM;
