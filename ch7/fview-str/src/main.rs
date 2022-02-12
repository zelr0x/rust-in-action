use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;
const INPUT: &'static [u8] = br#"
fn main() {
    println!("Hello, world!);
}
"#;

fn main() -> std::io::Result<()> {
    let mut buffer: Vec<u8> = vec!();
    INPUT.read_to_end(&mut buffer)?;
    
    let mut input_pos = 0;
    for line in buffer.chunks(BYTES_PER_LINE) {
        print!("[0x{:08x}] ", input_pos);
        for byte in line {
            print!("{:02x} ", byte);
        }
        println!();
        input_pos += BYTES_PER_LINE;
    }
    Ok(())
}

