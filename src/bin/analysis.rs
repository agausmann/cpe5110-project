use cpe5110_project::{
    booth3, booth4,
    util::{ceiling_div, SizedBinary, SizedHex},
};

/// (multiplier, multiplicand, bits)
const TEST_DATA: &[(i32, i32, u32)] = &[
    (0b1110, 0b1111, 4),
    (0b0101, 0b0101, 4),
    (0b111111, 0b111111, 6),
    (0b101110, 0b110111, 6),
    (0b111011, 0b100011, 6),
    (0b00011111, 0b01010101, 8),
    (0b11010111, 0b01010101, 8),
    (0b01010101, 0b11010111, 8),
    (0b01110111, 0b00110011, 8),
    (0b01111000, 0b01110111, 8),
    (0b0101010101, 0b0101010101, 10),
    (0b1100111011, 0b1001110000, 10),
    (0b1001101110, 0b0101111010, 10),
    (0b010101010101, 0b010101010101, 12),
    (0b001111100111, 0b000011111111, 12),
    (0b101010101010, 0b101010101010, 12),
    (0b111001110000, 0b000011111111, 12),
];

fn main() {
    println!(",,,,,Booth3,,,Booth4,,");
    println!("Multiplier (bin),Multiplicand (bin),Bits,Product (bin),Product (hex),Iterations,Additions,Gate Delay,Iterations,Additions,Gate Delay");
    for &(b, a, n) in TEST_DATA {
        let results_3 = booth3(a, b, n);
        let results_4 = booth4(a, b, n);
        let output_bits = 2 * n;
        assert_eq!(results_3.product, results_4.product);

        println!(
            "{},{},{},{},{},{},{},{},{},{},{}",
            SizedBinary(b, n),
            SizedBinary(a, n),
            n,
            SizedBinary(results_3.product, output_bits),
            SizedHex(results_3.product, ceiling_div(output_bits, 4)),
            results_3.iterations,
            results_3.additions,
            results_3.delay,
            results_4.iterations,
            results_4.additions,
            results_4.delay,
        );
    }
}
