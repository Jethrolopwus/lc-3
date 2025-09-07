# LC-3 Virtual Machine Implementation in Rust

A complete, modular implementation of the LC-3 (Little Computer 3) virtual machine written in Rust. This project provides a fully functional LC-3 processor emulator with all standard instructions, memory management, and register operations.

## Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [File Structure](#file-structure)
- [Core Components](#core-components)
- [Instruction Set](#instruction-set)
- [Usage](#usage)
- [Examples](#examples)
- [Technical Details](#technical-details)

## Overview

The LC-3 is a simplified computer architecture designed for educational purposes. This implementation provides:

- **Complete LC-3 ISA**: All 15 instruction types implemented
- **64KB Memory Space**: Full 16-bit addressable memory
- **8 General-Purpose Registers**: R0-R7 plus special registers (PC, COND)
- **Modular Design**: Clean separation of concerns with Rust's type system
- **Error Handling**: Comprehensive error handling with Result types
- **Debugging Support**: Built-in debugging and state inspection

## Architecture

The virtual machine follows a modular architecture with clear separation of concerns:

```
┌─────────────────┐
│   LC3VM (Main)  │ ← Coordinates all components
├─────────────────┤
│  RegisterFile   │ ← Manages all registers
├─────────────────┤
│     Memory      │ ← 64KB memory space
├─────────────────┤
│ InstructionExec │ ← Executes instructions
├─────────────────┤
│    Opcodes      │ ← Instruction definitions
└─────────────────┘
```

## File Structure

### Core Files

#### `Cargo.toml`
The Rust package configuration file that defines:
- **Package name**: "Lc3"
- **Version**: 0.1.0
- **Edition**: 2024 (latest Rust edition)
- **Dependencies**: Currently no external dependencies (pure Rust implementation)

#### `src/main.rs` - Entry Point and Demo
The main executable that demonstrates the VM capabilities:

```rust
// Key features demonstrated:
- VM initialization with test programs
- Instruction execution and debugging
- Register state inspection
- Opcode analysis and identification
- Memory operations
```

**What it does:**
1. Creates a new LC-3 virtual machine instance
2. Loads a simple test program (ST instruction)
3. Executes the program step-by-step
4. Displays VM state before and after execution
5. Demonstrates opcode extraction and analysis
6. Shows register values and VM statistics

#### `src/lib.rs` - Library Interface
The library's public interface that re-exports all main types:

```rust
// Re-exports for easy access:
pub use registers::{RegisterFile, Registers, Flags};
pub use memory::Memory;
pub use opcodes::Opcodes;
pub use instructions::{InstructionExecutor, ExecutionResult};
pub use vm::LC3VM;
```

**Purpose**: Provides a clean API for external users of the library.

### Core Components

#### `src/vm.rs` - Virtual Machine Core
The main VM structure that coordinates all components:

**Key Features:**
- **State Management**: Tracks running state and instruction count
- **Program Loading**: Initializes VM with programs at specified addresses
- **Execution Control**: Step-by-step or continuous execution
- **Debug Support**: Comprehensive state inspection

**Main Methods:**
```rust
impl LC3VM {
    pub fn new() -> Self                    // Create new VM
    pub fn initialize(start, program)       // Load program
    pub fn step() -> ExecutionResult        // Execute one instruction
    pub fn run() -> Result<(), String>      // Run until halt
    pub fn run_for(max_instructions)        // Run for N instructions
    pub fn debug_info() -> String           // Get VM state
}
```

**Human Understanding:**
Think of `LC3VM` as the "brain" of our computer. It:
- Keeps track of whether the computer is running or stopped
- Counts how many instructions have been executed
- Coordinates between memory, registers, and instruction execution
- Provides a simple interface to load and run programs

#### `src/registers.rs` - Register Management
Manages all LC-3 registers including general-purpose and special registers:

**Register Types:**
- **R0-R7**: 8 general-purpose registers for data storage
- **PC**: Program Counter (points to current instruction)
- **COND**: Condition Code Register (stores comparison results)

**Condition Flags:**
- **POS**: Positive (value > 0)
- **ZRO**: Zero (value = 0)  
- **NEG**: Negative (value < 0)

**Key Features:**
```rust
impl RegisterFile {
    pub fn read(reg) -> Option<u16>         // Read register value
    pub fn write(reg, value)                // Write to register
    pub fn update_condition_code(value)     // Update COND flags
    pub fn get_pc() -> u16                  // Get program counter
    pub fn set_pc(value)                    // Set program counter
}
```

**Human Understanding:**
Registers are like the computer's "workspace" - small, fast storage locations:
- **R0-R7** are like 8 small notepads where you can write numbers
- **PC** is like a bookmark showing which instruction to execute next
- **COND** is like a status light showing if the last result was positive, zero, or negative

#### `src/memory.rs` - Memory System
Manages the 64KB memory space of the LC-3:

**Memory Layout:**
- **Size**: 65,536 words (64KB)
- **Word Size**: 16 bits per word
- **Addressable Range**: 0x0000 to 0xFFFF

**Key Features:**
```rust
impl Memory {
    pub fn read(address) -> Option<u16>     // Read from memory
    pub fn write(address, value)            // Write to memory
    pub fn load_program(start, program)     // Load program
    pub fn fetch_instruction(registers)     // Get next instruction
}
```

**Human Understanding:**
Memory is like a huge filing cabinet with 65,536 drawers:
- Each drawer can hold one 16-bit number
- You can read from or write to any drawer by its address
- Programs are stored in memory as sequences of instructions
- The VM reads instructions from memory one by one

#### `src/opcodes.rs` - Instruction Definitions
Defines all LC-3 instruction types and bit manipulation utilities:

**Instruction Types:**
```rust
pub enum Opcodes {
    BR,   // Branch (conditional jump)
    ADD,  // Add two values
    LD,   // Load from memory
    ST,   // Store to memory
    JSR,  // Jump to subroutine
    AND,  // Bitwise AND
    LDR,  // Load with base+offset
    STR,  // Store with base+offset
    RTI,  // Return from interrupt
    NOT,  // Bitwise NOT
    LDI,  // Load indirect
    STI,  // Store indirect
    JMP,  // Jump to register
    RES,  // Reserved
    LEA,  // Load effective address
    TRAP, // System call
}
```

**Bit Extraction Functions:**
- `extract_opcode()`: Gets instruction type (bits 15-12)
- `extract_dr()`: Gets destination register (bits 11-9)
- `extract_sr1()`: Gets first source register (bits 8-6)
- `extract_imm5()`: Gets 5-bit immediate value
- `sign_extend_*()`: Extends signed values to 16 bits

**Human Understanding:**
Each instruction is a 16-bit number that tells the computer what to do:
- The first 4 bits (opcode) say "what operation" (ADD, STORE, etc.)
- Other bits specify "which registers" and "what values"
- Bit extraction functions are like reading different parts of a form

#### `src/instructions.rs` - Instruction Execution
Implements the execution logic for all LC-3 instructions:

**Execution Flow:**
1. **Fetch**: Get instruction from memory
2. **Decode**: Extract opcode and operands
3. **Execute**: Perform the operation
4. **Update**: Modify registers/memory as needed

**Implemented Instructions:**

**Arithmetic & Logic:**
- **ADD**: `R[dr] = R[sr1] + R[sr2]` or `R[dr] = R[sr1] + imm5`
- **AND**: `R[dr] = R[sr1] & R[sr2]` or `R[dr] = R[sr1] & imm5`
- **NOT**: `R[dr] = ~R[sr1]`

**Memory Operations:**
- **LD**: `R[dr] = M[PC + offset9]` (load from memory)
- **ST**: `M[PC + offset9] = R[sr]` (store to memory)
- **LDR**: `R[dr] = M[R[base] + offset6]` (load with base+offset)
- **STR**: `M[R[base] + offset6] = R[sr]` (store with base+offset)
- **LDI**: `R[dr] = M[M[PC + offset9]]` (indirect load)
- **STI**: `M[M[PC + offset9]] = R[sr]` (indirect store)
- **LEA**: `R[dr] = PC + offset9` (load effective address)

**Control Flow:**
- **BR**: Conditional branch based on condition codes
- **JSR**: Jump to subroutine (save return address in R7)
- **JMP**: Unconditional jump to register address

**System Operations:**
- **TRAP**: Execute system calls (GETC, OUT, PUTS, IN, PUTSP, HALT)

**Human Understanding:**
Each instruction is like a command that tells the computer to:
- **ADD**: "Take two numbers and add them together"
- **STORE**: "Put this number into memory at this location"
- **BRANCH**: "If the last result was zero, jump to this instruction"
- **TRAP**: "Do a system operation like print to screen"

## Instruction Set

### Complete LC-3 Instruction Reference

| Opcode | Mnemonic | Description | Format |
|--------|----------|-------------|---------|
| 0x0 | BR | Branch conditionally | `BR[nzp] offset9` |
| 0x1 | ADD | Add two values | `ADD DR SR1 SR2` or `ADD DR SR1 imm5` |
| 0x2 | LD | Load from memory | `LD DR offset9` |
| 0x3 | ST | Store to memory | `ST SR offset9` |
| 0x4 | JSR | Jump to subroutine | `JSR offset11` or `JSRR base` |
| 0x5 | AND | Bitwise AND | `AND DR SR1 SR2` or `AND DR SR1 imm5` |
| 0x6 | LDR | Load with base+offset | `LDR DR base offset6` |
| 0x7 | STR | Store with base+offset | `STR SR base offset6` |
| 0x8 | RTI | Return from interrupt | `RTI` |
| 0x9 | NOT | Bitwise NOT | `NOT DR SR` |
| 0xA | LDI | Load indirect | `LDI DR offset9` |
| 0xB | STI | Store indirect | `STI SR offset9` |
| 0xC | JMP | Jump to register | `JMP base` |
| 0xD | RES | Reserved | - |
| 0xE | LEA | Load effective address | `LEA DR offset9` |
| 0xF | TRAP | System call | `TRAP trapvect8` |

### TRAP Vectors

| Vector | Name | Description |
|--------|------|-------------|
| 0x20 | GETC | Get character from keyboard |
| 0x21 | OUT | Output character |
| 0x22 | PUTS | Output string |
| 0x23 | IN | Input character and echo |
| 0x24 | PUTSP | Output string (2 chars/word) |
| 0x25 | HALT | Halt the program |

## Usage

### Basic Usage

```rust
use Lc3::*;

// Create a new VM
let mut vm = LC3VM::new();

// Create a simple program
let program = vec![
    0x3000, // ST instruction
];

// Initialize and run
vm.initialize(0x3000, &program)?;
vm.run()?;
```

### Step-by-Step Execution

```rust
// Execute one instruction at a time
while vm.is_running() {
    match vm.step() {
        Ok(ExecutionResult::Continue) => {
            println!("Instruction executed");
        }
        Ok(ExecutionResult::Halt) => {
            println!("Program halted");
            break;
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            break;
        }
    }
}
```

### Debugging

```rust
// Get VM state
println!("{}", vm.debug_info());

// Check specific registers
let pc = vm.get_pc();
let r0 = vm.get_register(Registers::R0);

// Read memory
let value = vm.read_memory(0x3000);
```

## Examples

### Example 1: Simple Addition

```rust
// Program that adds 5 + 3 = 8
let program = vec![
    0xE005, // LEA R0, #5 (load 5 into R0)
    0xE203, // LEA R1, #3 (load 3 into R1)  
    0x1041, // ADD R0, R0, R1 (R0 = R0 + R1)
    0xF025, // TRAP HALT
];

vm.initialize(0x3000, &program)?;
vm.run()?;

// R0 should now contain 8
let result = vm.get_register(Registers::R0).unwrap();
println!("5 + 3 = {}", result);
```

### Example 2: Memory Operations

```rust
// Program that stores and loads values
let program = vec![
    0xE005, // LEA R0, #5
    0x3000, // ST R0, #0 (store R0 at PC+0)
    0x2000, // LD R0, #0 (load from PC+0 into R0)
    0xF025, // TRAP HALT
];

vm.initialize(0x3000, &program)?;
vm.run()?;
```

### Example 3: Conditional Branching

```rust
// Program with conditional logic
let program = vec![
    0xE000, // LEA R0, #0 (R0 = 0)
    0x0401, // BRz #1 (if zero, skip next instruction)
    0xE001, // LEA R0, #1 (R0 = 1)
    0xF025, // TRAP HALT
];

vm.initialize(0x3000, &program)?;
vm.run()?;
// R0 should be 0 (branch taken)
```

## Technical Details

### Memory Layout

```
Address Range    | Purpose
-----------------|------------------
0x0000 - 0x2FFF  | System/Reserved
0x3000 - 0xFDFF  | User Program Space
0xFE00 - 0xFFFF  | I/O Device Registers
```

### Register Layout

```
Register | Purpose
---------|------------------
R0 - R7  | General Purpose
PC       | Program Counter
COND     | Condition Codes
```

### Instruction Format

All LC-3 instructions are 16 bits wide:

```
15 14 13 12 11 10 9  8  7  6  5  4  3  2  1  0
[  Opcode  ][  Operands vary by instruction  ]
```

### Error Handling

The implementation uses Rust's `Result<T, E>` type for comprehensive error handling:

- **Memory errors**: Out-of-bounds access
- **Register errors**: Invalid register access
- **Execution errors**: Invalid instructions or runtime errors

### Performance Characteristics

- **Memory**: 64KB with O(1) access time
- **Registers**: 10 registers with O(1) access time
- **Instruction execution**: Single-cycle for most instructions
- **No pipelining**: Sequential instruction execution

## Building and Running

```bash
# Build the project
cargo build

# Run the demo
cargo run

# Run tests (if any)
cargo test

# Build for release
cargo build --release
```

## Educational Value

This implementation serves as an excellent learning resource for:

- **Computer Architecture**: Understanding how processors work
- **Assembly Language**: Learning low-level programming concepts
- **Virtual Machines**: Understanding emulation and interpretation
- **Rust Programming**: Demonstrating systems programming in Rust
- **Software Design**: Modular architecture and clean interfaces

## Future Enhancements

Potential improvements could include:

- **I/O System**: Implement keyboard and display interfaces
- **Assembler**: Convert assembly language to machine code
- **Debugger**: Step-through debugging capabilities
- **Performance**: Instruction pipelining and optimization
- **Extensions**: Additional instruction set extensions

---

This LC-3 implementation provides a complete, educational virtual machine that demonstrates fundamental computer architecture concepts while showcasing clean, idiomatic Rust code.
