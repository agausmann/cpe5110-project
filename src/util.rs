use std::fmt;

/// A format wrapper that prints the lower n bits of a value.
pub struct SizedBinary(pub i32, pub u32);

impl fmt::Display for SizedBinary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for k in (0..self.1).rev() {
            write!(f, "{}", (self.0 >> k) & 0b1)?;
        }
        Ok(())
    }
}

/// A format wrapper that prints the lower n hex digits of a value.
pub struct SizedHex(pub i32, pub u32);

impl fmt::Display for SizedHex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for k in (0..self.1).rev() {
            write!(f, "{}", (self.0 >> (4 * k)) & 0xf)?;
        }
        Ok(())
    }
}

pub fn register_mask(bits: u32) -> i32 {
    (1 << bits) - 1
}

pub fn sign_extend(x: i32, n: u32) -> i32 {
    let sign_mask = -1 & !register_mask(n);
    if x & (1 << (n - 1)) == 0 {
        x & !sign_mask
    } else {
        x | sign_mask
    }
}

pub fn ceiling_div(a: u32, b: u32) -> u32 {
    1 + (a - 1) / b
}
