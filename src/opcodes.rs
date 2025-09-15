
pub use crate::types::{
    Opcodes, TrapVectors,
    extract_opcode, extract_dr, extract_sr1, extract_sr2,
    extract_imm5_flag, extract_imm5, extract_pc_offset9, extract_pc_offset11,
    extract_offset6, extract_trap_vector,
    sign_extend_imm5, sign_extend_offset6, sign_extend_pc_offset9,
    sign_extend_pc_offset11, sign_extend,
};
