fn main() {
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();  // allow bit maniputation
    let sign_bit = n_bits >> 31;

    let exponent_ = n_bits >> 23;   // move the sign bit and the 8-bit exponent to the right
                                    // overwriting the mantissa
    let exponent_ = exponent_ & 0xff;   // discard the sign bit
    let exponent = (exponent_ as i32) - 127;    // interpret bits as signed and substruct the bias
                                                // as defined by the standard

    let mut mantissa: f32 = 1.0;  // 2^-0 - the weight of the implicit 24-th bit
    for i in 0..23 {
        let mask = 1 << i; // e.g. if i == 5, mask is 0b00000000_00000000_00000000_00100000
        let one_at_bit_i = n_bits & mask; // 0 iff 0 at bit i in the original number
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0); // weight of the bit at i: 2^(i-23)
            mantissa += weight;
        }
    }
}
