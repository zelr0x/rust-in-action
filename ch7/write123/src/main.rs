use std::io::Cursor; // Enables a Vec<T> to mock being a file.
use byteorder::{LittleEndian}; // Used as a type argument for read_* / write_* methods.
use byteorder::{ReadBytesExt, WriteBytesExt}; // Traits that provide read_* / write_* methods.

fn write_numbers_to_file() -> (u32, i8, f64) {
    let mut w = vec![]; // writer
    
    let one: u32   = 1;
    let two: i8    = 2;
    let three: f64 = 3.0;

    w.write_u32::<LittleEndian>(one).unwrap(); // Won't fail unless something is srsly wrong.
    println!("{:?}", &w);

    w.write_i8(two).unwrap(); // i8 is single byte so it doesn't need an endianness parameter.
    println!("{:?}", &w);

    w.write_f64::<LittleEndian>(three).unwrap();
    println!("{:?}", &w);

    (one, two, three)
}

fn read_numbers_from_file() -> (u32, i8, f64) {
    let mut r = Cursor::new(vec![1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 8, 64]);
    let one = r.read_u32::<LittleEndian>().unwrap();
    let two = r.read_i8().unwrap();
    let three = r.read_f64::<LittleEndian>().unwrap();
    (one, two, three)
}

fn main() {
    let (one, two, three) = write_numbers_to_file();
    let (one_, two_, three_) = read_numbers_from_file();

    assert_eq!(one, one_);
    assert_eq!(two, two_);
    assert_eq!(three, three_);
}

