use Lc3::*;
use Lc3::opcodes::{extract_opcode, Opcodes};


const PC_START: u16 = 0x3000;

fn main() {
    // Create a new LC-3 virtual machine
    let mut vm = LC3VM::new();
    
    
    let test_program = vec![
        0x3000,
    ];
    
  
    match vm.initialize(PC_START, &test_program) {
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
    
    // Execute one instruction to demonstrate the modular structure
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
    println!("COND: 0x{:04X}", vm.get_register(Registers::COND).unwrap_or(0));
    
    println!("\nVM Statistics:");
    println!("Instructions executed: {}", vm.get_instruction_count());
    println!("VM running: {}", vm.is_running());
}