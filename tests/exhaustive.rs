use cpe5110_project::{booth3, booth4, Results};

fn test_exhaustive(booth: fn(i32, i32, u32) -> Results, bits: u32) {
    let start = -2i32.pow(bits - 1);
    let end = 2i32.pow(bits - 1);
    for a in start..end {
        for b in start..end {
            assert_eq!(a * b, booth(a, b, bits).product, "operands {}, {}", a, b);
        }
    }
}

#[test]
fn booth3_4bit_exhaustive() {
    test_exhaustive(booth3, 4);
}

#[test]
fn booth3_6bit_exhaustive() {
    test_exhaustive(booth3, 6);
}

#[test]
fn booth3_8bit_exhaustive() {
    test_exhaustive(booth3, 8);
}

#[test]
fn booth3_10bit_exhaustive() {
    test_exhaustive(booth3, 10);
}

#[test]
fn booth3_12bit_exhaustive() {
    test_exhaustive(booth3, 12);
}

#[test]
fn booth4_4bit_exhaustive() {
    test_exhaustive(booth4, 4);
}

#[test]
fn booth4_6bit_exhaustive() {
    test_exhaustive(booth4, 6);
}

#[test]
fn booth4_8bit_exhaustive() {
    test_exhaustive(booth4, 8);
}

#[test]
fn booth4_10bit_exhaustive() {
    test_exhaustive(booth4, 10);
}

#[test]
fn booth4_12bit_exhaustive() {
    test_exhaustive(booth4, 12);
}
