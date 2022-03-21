pub mod util;

use util::{ceiling_div, register_mask, sign_extend};

pub struct Results {
    pub product: i32,
    pub iterations: u32,
    pub additions: u32,
    pub delay: u32,
}

pub fn booth3(a: i32, b: i32, n: u32) -> Results {
    let a = sign_extend(a, n);
    let b = sign_extend(b, n);

    // Additional counters for results:
    let mut additions = 0;
    let mut delay = 0;

    // Multiples of A are precomputed.
    // Longest time is -2*A which is just shift and complement,
    // plus mux and buffer
    delay += complement_delay(n) + mux_delay(3) + LATCH_DELAY;

    // 1. Shift B left, inserting zero for LSB.
    let mut b = b << 1;

    // 2. Let PQ = 0 (double register).
    let mut pq = DoubleRegister {
        value: 0,
        low_bits: n,
    };

    // 3. Let m = ceil((n + 1) / 2).
    let m = ceiling_div(n + 1, 2);

    // 4. Let k = 0.
    let mut k = 0;

    loop {
        // 5. Shift PQ right 2 bits, preserving sign.
        pq.value >>= 2;

        // 6. Match against the lower 3 bits of B:
        let coefficient = match b & 0b111 {
            0b000 => 0,
            0b001 => 1,
            0b010 => 1,
            0b011 => 2,
            0b100 => -2,
            0b101 => -1,
            0b110 => -1,
            0b111 => 0,
            _ => unreachable!(),
        };
        let add_delay = carry_select_delay(n) + mux_delay(1);
        let no_add_delay = mux_delay(1);
        let lookup_delay = mux_delay(4);
        if coefficient != 0 {
            // ... Add x * A to P:
            pq.set_high(pq.high() + coefficient * a);
            // (Selection for next group of bits is done
            // in parallel with the addition for this group:)
            delay += add_delay.max(lookup_delay);
            additions += 1;
        } else {
            delay += no_add_delay.max(lookup_delay);
        }

        // 7. Shift B right 2 bits, preserving sign.
        b >>= 2;

        // 8. Increment k.
        k += 1;

        // 9. If k < m, repeat from step 5.
        if k < m {
            // Shift and latch:
            delay += LATCH_DELAY;
            continue;
        } else {
            break;
        }
    }

    // 10. Shift PQ right (n mod 2) bits, preserving sign.
    pq.value >>= n % 2;

    // 11. Return the contents of PQ.
    Results {
        product: pq.value,
        iterations: m,
        additions,
        delay,
    }
}

pub fn booth4(a: i32, b: i32, n: u32) -> Results {
    let a = sign_extend(a, n);
    let b = sign_extend(b, n);

    // Additional counters for results:
    let mut additions = 0;
    let mut delay = 0;

    // Multiples of A are precomputed.
    // Longest time is -3*A which has 2 addition terms, so
    // use a carry select adder, plus complement, mux, and buffer:
    delay += carry_select_delay(n) + complement_delay(n) + mux_delay(4) + LATCH_DELAY;

    // 1. Shift B left, inserting zero for LSB.
    let mut b = b << 1;

    // 2. Let PQ = 0 (double register).
    let mut pq = DoubleRegister {
        value: 0,
        low_bits: n,
    };

    // 3. Let m = ceil((n + 1) / 3).
    let m = ceiling_div(n + 1, 3);

    // 4. Let k = 0.
    let mut k = 0;

    loop {
        // 5. Shift PQ right 3 bits, preserving sign.
        pq.value >>= 3;

        // 6. Match against the lower 4 bits of B:
        let coefficient = match b & 0b1111 {
            0b0000 => 0,
            0b0001 => 1,
            0b0010 => 1,
            0b0011 => 2,
            0b0100 => 2,
            0b0101 => 3,
            0b0110 => 3,
            0b0111 => 4,
            0b1000 => -4,
            0b1001 => -3,
            0b1010 => -3,
            0b1011 => -2,
            0b1100 => -2,
            0b1101 => -1,
            0b1110 => -1,
            0b1111 => 0,
            _ => unreachable!(),
        };
        let add_delay = carry_select_delay(n) + mux_delay(1);
        let no_add_delay = mux_delay(1);
        let lookup_delay = mux_delay(4);
        if coefficient != 0 {
            // ... Add x * A to P:
            pq.set_high(pq.high() + coefficient * a);
            // (Selection for next group of bits is done
            // in parallel with the addition for this group:)
            delay += add_delay.max(lookup_delay);
            additions += 1;
        } else {
            delay += no_add_delay.max(lookup_delay);
        }

        // 7. Shift B right 3 bits, preserving sign.
        b >>= 3;

        // 8. Increment k.
        k += 1;

        // 9. If k < m, repeat from step 5.
        if k < m {
            // Shift and latch:
            delay += 3;
            continue;
        } else {
            break;
        }
    }

    // 10. Shift PQ right (n mod 3) bits, preserving sign.
    pq.value >>= n % 3;

    // 11. Return the contents of PQ.
    Results {
        product: pq.value,
        iterations: m,
        additions,
        delay,
    }
}

struct DoubleRegister {
    value: i32,
    low_bits: u32,
}

impl DoubleRegister {
    fn high(&self) -> i32 {
        self.value >> self.low_bits
    }

    fn set_high(&mut self, value: i32) {
        self.value &= register_mask(self.low_bits);
        self.value |= value << self.low_bits;
    }
}

fn carry_select_delay(bits: u32) -> u32 {
    match bits {
        4 => 8,   // 2-2
        6 => 10,  // 3-3
        8 => 12,  // 4-4
        10 => 12, // 4-3-3
        12 => 14, // 4-4-4
        _ => unimplemented!("{} bit adder", bits),
    }
}

fn mux_delay(selector_bits: u32) -> u32 {
    selector_bits + 1
}

fn complement_delay(n: u32) -> u32 {
    n
}

const LATCH_DELAY: u32 = 3;
