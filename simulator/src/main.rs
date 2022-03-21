use std::io::{stdin, stdout, Write};

use cpe5110_project::{
    booth3, booth4,
    util::{ceiling_div, SizedBinary, SizedHex},
    Results,
};

macro_rules! unwrap_or_break {
    ($option:expr) => {
        match $option {
            Some(x) => x,
            None => break,
        }
    };
}

fn main() -> std::io::Result<()> {
    loop {
        println!();
        let booth_fn = unwrap_or_break!(prompt_and_then(
            "Group size (3 or 4): ",
            |line| match line.trim() {
                "3" => Some(booth3 as fn(i32, i32, u32) -> Results),
                "4" => Some(booth4 as fn(i32, i32, u32) -> Results),
                _ => None,
            }
        )?);

        let a = unwrap_or_break!(prompt_binary_i32("Multiplicand (bin): ")?);
        let b = unwrap_or_break!(prompt_binary_i32("Multiplier (bin): ")?);
        let n = unwrap_or_break!(prompt_and_then("Bits (dec): ", |line| line
            .trim()
            .parse::<u32>()
            .ok())?);

        let results = booth_fn(a, b, n);
        let output_bits = 2 * n;
        println!();
        println!(
            "Product (bin): {}",
            SizedBinary(results.product, output_bits)
        );
        println!(
            "Product (hex): {}",
            SizedHex(results.product, ceiling_div(output_bits, 4))
        );
        println!("Iterations (dec): {}", results.iterations);
        println!("Additions (dec): {}", results.additions);
        println!("Gate delay (dec): {}", results.delay);
        println!();
        println!("----------");
    }
    println!();

    Ok(())
}

fn prompt(s: &str) -> std::io::Result<Option<String>> {
    print!("{}", s);
    stdout().flush()?;
    let mut line = String::new();
    let n = stdin().read_line(&mut line)?;
    if n == 0 {
        Ok(None)
    } else {
        Ok(Some(line))
    }
}

fn prompt_and_then<F, T>(s: &str, mut mapper: F) -> std::io::Result<Option<T>>
where
    F: FnMut(&str) -> Option<T>,
{
    loop {
        let line = match prompt(s)? {
            Some(x) => x,
            None => return Ok(None),
        };
        match mapper(&line) {
            Some(x) => return Ok(Some(x)),
            None => continue,
        }
    }
}

fn prompt_binary_i32(s: &str) -> std::io::Result<Option<i32>> {
    prompt_and_then(s, |line| i32::from_str_radix(line.trim(), 2).ok())
}
