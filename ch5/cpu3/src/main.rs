//! CHIP-8 CPU emulator.

/// The emulated central processing unit.
///
/// Operation codes (opcodes):
/// 0x0000 - HALT: stop execution.
/// 0x2nnn - CALL: set the program counter to the memory address `nnn`.
/// 0x00EE - RETURN: set the program counter to the memory address of the previous `CALL` opcode.
/// 0x8XY4 - ADD: add the value of the register `Y` to the value of the register `X`
///          and store the result in the register `X`.
///
/// `registers` - a set of emulated CPU's registers.
/// The last register (0xF) is used as a carry flag. When set, it indicates that an operation
/// has overflowed the u8 register size.
///
/// `pc` - a program counter (position in memory).
///
/// `sp` - a stack pointer.
///
/// `stack` - 16-address-deep stack, stack overflow after 16 nested function calls.
///
/// `memory` - 4KB of RAM, placed into this struct for convenience.
/// The first 512B (0x100) are reserved for the system in CHIP-8, but not in this emulator.
struct CPU {
    registers: [u8; 16],
    pc: usize, // should be u16
    sp: usize, // should be u16
    stack: [u16; 16],
    memory: [u8; 0x1000],
}

impl CPU {
    fn new() -> CPU {
        CPU {
            registers: [0; 16],
            pc: 0,
            sp: 0,
            stack: [0; 16],
            memory: [0; 4096],
        }
    }

    fn read_opcode(&self) -> u16 {
        let p = self.pc;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        op_byte1 << 8 | op_byte2 // shift op_byte1 to the left and combine with op_byte2 using OR
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.pc += 2; // increment PC to go to the next instruction
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >>  8) as u8;
            let y = ((opcode & 0x00F0) >>  4) as u8;
            let d = ((opcode & 0x000F) >>  0) as u8;
            let nnn = opcode & 0x0FFF;
            // let k = (opcode & 0x00FF) as u8;
            match (c, x, y, d) {
                (  0,   0,   0,   0) => { return; },
                (  0,   0, 0xE, 0xE) => self.ret(),
                (0x2,   _,   _,   _) => self.call(nnn),
                (0x8,   _,   _, 0x4) => self.add_xy(x, y),
                _                => todo!("opcode {:04x}", opcode),
            }
        }
    }
    
    fn call(&mut self, addr: u16) {
        let sp = self.sp;
        let stack = &mut self.stack;
        if sp > stack.len() {
            panic!("Stack overflow!");
        }
        // add the PC to stack; this address is two bytes higher than
        // the calling location as it is incremented within `run`
        stack[sp] = self.pc as u16;
        self.sp += 1; // increment the SP to prevent PC stored there from being overwritten
        self.pc = addr as usize;
    }

    fn ret(&mut self) {
        if self.sp == 0 {
            panic!("Stack overflow");
        }
        self.sp -= 1;
        let call_addr = self.stack[self.sp];
        self.pc = call_addr as usize;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;
        self.registers[0xF] = if overflow { 1 } else { 0 };
    }
}

fn main() {
    let mut cpu = CPU::new();
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    let mem = &mut cpu.memory;
    mem[0x000] = 0x21; mem[0x001] = 0x00; // load opcode 0x2100: CALL 0x100
    mem[0x002] = 0x21; mem[0x003] = 0x00; // load opcode 0x2100: CALL 0x100
    mem[0x004] = 0x00; mem[0x005] = 0x00; // load opcode 0x0000: HALT (not strictly necessary
                                          // as the memory is initialized with null bytes)
    mem[0x100] = 0x80; mem[0x101] = 0x14; // load opcode 0x8014: ADD reg0 reg1
    mem[0x102] = 0x80; mem[0x103] = 0x14; // load opcode 0x8014: ADD reg0 reg1
    mem[0x104] = 0x00; mem[0x105] = 0xEE; // load opcode 0x00EE: RET 

    cpu.run();
    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}

