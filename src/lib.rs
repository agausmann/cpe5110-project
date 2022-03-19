pub struct Results {
    pub product: i32,
    pub iterations: u32,
    pub additions: u32,
    pub delay: u32,
}

pub fn booth3(a: i32, b: i32, n: u32) -> Results {
    // Additional counters for results:
    let mut additions = 0;
    let mut delay = 0;

    // Multiples of A are precomputed.
    // Longest time is -3*A which needs an addition:
    delay += carry_select_delay(n) + complement_delay(n);

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
        // 5. Shift PQ right 2 bits, preserving sign.
        pq.value >>= 2;

        // 6. Match against the lower 3 bits of B:
        let coefficient = match b & 0b111 {
            0b000 => 0,
            0b001 => 1,
            0b010 => 2,
            0b011 => 3,
            0b100 => -3,
            0b101 => -2,
            0b110 => -1,
            0b111 => 0,
            _ => unreachable!(),
        };
        if coefficient != 0 {
            // ... Add x * A to P:
            pq.set_high(pq.high() + coefficient * a);
            // (Selection for next group of bits is done
            // in parallel with the addition for this group:)
            delay += carry_select_delay(n).max(mux_delay(3));
            additions += 1;
        } else {
            delay += mux_delay(3);
        }

        // 7. Shift B right 2 bits, preserving sign.
        b >>= 2;

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

    // 10. Shift PQ right ((n + 1) mod 3) bits, preserving sign.
    pq.value >>= (n + 1) % 3;

    // 11. Return the contents of PQ.
    Results {
        product: pq.value,
        iterations: m,
        additions,
        delay,
    }
}

pub fn booth4(a: i32, b: i32, n: u32) -> Results {
    // Additional counters for results:
    let mut additions = 0;
    let mut delay = 0;

    // Multiples of A are precomputed.
    // Longest time is -7*A which has 3 addition terms,
    // so use carry save plus a 2-term carry select:
    delay += 4 + carry_select_delay(n) + complement_delay(n);

    // 1. Shift B left, inserting zero for LSB.
    let mut b = b << 1;

    // 2. Let PQ = 0 (double register).
    let mut pq = DoubleRegister {
        value: 0,
        low_bits: n,
    };

    // 3. Let m = ceil((n + 1) / 4).
    let m = ceiling_div(n + 1, 4);

    // 4. Let k = 0.
    let mut k = 0;

    loop {
        // 5. Shift PQ right 3 bits, preserving sign.
        pq.value >>= 3;

        // 6. Match against the lower 4 bits of B:
        let coefficient = match b & 0b1111 {
            0b0000 => 0,
            0b0001 => 1,
            0b0010 => 2,
            0b0011 => 3,
            0b0100 => 4,
            0b0101 => 5,
            0b0110 => 6,
            0b0111 => 7,
            0b1000 => -7,
            0b1001 => -6,
            0b1010 => -5,
            0b1011 => -4,
            0b1100 => -3,
            0b1101 => -2,
            0b1110 => -1,
            0b1111 => 0,
            _ => unreachable!(),
        };
        if coefficient != 0 {
            // ... Add x * A to P:
            pq.set_high(pq.high() + coefficient * a);
            // (Selection for next group of bits is done
            // in parallel with the addition for this group:)
            delay += carry_select_delay(n).max(mux_delay(4));
            additions += 1;
        } else {
            delay += mux_delay(4);
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

    // 10. Shift PQ right ((n + 1) mod 4) bits, preserving sign.
    pq.value >>= (n + 1) % 4;

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

fn ceiling_div(a: u32, b: u32) -> u32 {
    1 + (a - 1) / b
}

fn register_mask(bits: u32) -> i32 {
    (1 << bits) - 1
}

fn carry_select_delay(bits: u32) -> u32 {
    match bits {
        6 => todo!(),
        8 => todo!(),
        10 => todo!(),
        12 => todo!(),
        _ => unimplemented!("{} bit adder", bits),
    }
}

fn mux_delay(selector_bits: u32) -> u32 {
    match selector_bits {
        3 => 2,
        4 => 2,
        _ => unimplemented!("{} bit mux", selector_bits),
    }
}

fn complement_delay(n: u32) -> u32 {
    n
}
