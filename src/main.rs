use lc3::instructions::InstructionExecutor;
use lc3::types::{Opcodes, PC_START, extract_opcode};
use lc3::*;

fn main() {
    let mut vm = LC3VM::new();

    let first_program = vec![0x3000];

    match vm.initialize(PC_START, &first_program) {
        Ok(_) => {
            println!("VM initialized successfully");
            println!("Program loaded at address 0x{:04X}", PC_START);
        }
        Err(e) => {
            eprintln!("Failed to initialize VM: {}", e);
            return;
        }
    }

    println!("\nInitial VM State:");
    println!("{}", vm.debug_info());

    match vm.step() {
        Ok(result) => {
            println!("\nInstruction executed successfully");
            println!("Execution result: {:?}", result);
        }
        Err(e) => {
            eprintln!("Error executing instruction: {}", e);
            return;
        }
    }

    println!("\nFinal VM State:");
    println!("{}", vm.debug_info());

    let instruction = vm.read_memory(PC_START).unwrap_or(0);
    let opcode = extract_opcode(instruction);

    println!("\nInstruction Analysis:");
    println!("Instruction: 0x{:04X}", instruction);
    println!("Opcode: {}", opcode);

    match Opcodes::from_u16(opcode) {
        Some(op) => {
            println!("Operation: {} - {}", op.to_string(), op.description());
        }
        None => {
            println!("Unknown opcode: {}", opcode);
        }
    }

    println!("\nRegister Values:");
    for i in 0..8 {
        let reg = Registers::from(i);
        let value = vm.get_register(reg).unwrap_or(0);
        println!("R{}: 0x{:04X} ({})", i, value, value);
    }

    println!("\nSpecial Registers:");
    println!("PC: 0x{:04X}", vm.get_pc());
    println!(
        "COND: 0x{:04X}",
        vm.get_register(Registers::COND).unwrap_or(0)
    );

    println!("\nVM Statistics:");
    println!("Instructions executed: {}", vm.get_instruction_count());
    println!("VM running: {}", vm.is_running());

    println!("\n{}", "=".repeat(60));
    println!("TESTING SIGN_EXTEND FUNCTION");

    let test_cases = vec![
        (0x1F, 5, "5-bit positive: 31 (0x1F)"),
        (0x10, 5, "5-bit negative: -16 (0x10)"),
        (0x3F, 6, "6-bit positive: 63 (0x3F)"),
        (0x20, 6, "6-bit negative: -32 (0x20)"),
        (0x1FF, 9, "9-bit positive: 511 (0x1FF)"),
        (0x100, 9, "9-bit negative: -256 (0x100)"),
        (0x7FF, 11, "11-bit positive: 2047 (0x7FF)"),
        (0x400, 11, "11-bit negative: -1024 (0x400)"),
    ];

    for (value, bit_count, description) in test_cases {
        println!("\nTest: {}", description);
        let result = InstructionExecutor::sign_extend(value, bit_count);
        println!(
            "Expected behavior: {} should become {}",
            if (value >> (bit_count - 1)) & 1 == 1 {
                "negative"
            } else {
                "positive"
            },
            result as i16
        );
    }
}
