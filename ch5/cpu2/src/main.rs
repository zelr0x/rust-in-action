//! CHIP-8 CPU emulator.

/// The emulated central processing unit.
///
/// Operation codes (opcodes):
/// 0x0000 - stop execution.
/// 0x8**4 - add, `*` are for the registers.
///
/// `registers` is a set of emulated CPU's registers.
/// The last register (0xF) is used as a carry flag. When set, it indicates that an operation
/// has overflowed the u8 register size.
///
/// `pc` is a program counter (position in memory).
///
/// `memory` - 4KB of RAM, placed into this struct for convenience.
/// The first 512B (0x100) are reserved for the system in CHIP-8, but not in this emulator.
struct CPU {
    registers: [u8; 16],
    pc: usize, // should be u16
    memory: [u8; 0x1000],
}

impl CPU {
    fn new() -> CPU {
        CPU {
            registers: [0; 16],
            memory: [0; 4096],
            pc: 0,
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
            match (c, x, y, d) {
                (0, 0, 0, 0)     => { return; },
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _                => todo!("opcode {:04x}", opcode),
            }
        }
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
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;
    let mem = &mut cpu.memory;
    mem[0] = 0x80; mem[1] = 0x14; // load opcode 0x8014 (add reg1 to reg0)
    mem[2] = 0x80; mem[3] = 0x24; // load opcode 0x8024 (add reg2 to reg0)
    mem[4] = 0x80; mem[5] = 0x34; // load opcode 0x8032 (add reg3 to reg0)

    cpu.run();
    assert_eq!(cpu.registers[0], 35);
    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}

