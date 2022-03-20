use std::io::{stdin, stdout, Write};

use cpe5110_project::{
    booth3,
    util::{ceiling_div, SizedBinary, SizedHex},
};

fn main() -> std::io::Result<()> {
    loop {
        println!();
        let a = match prompt_binary_i32("Multiplicand (bin): ")? {
            Some(x) => x,
            None => break,
        };
        let b = match prompt_binary_i32("Multiplier (bin): ")? {
            Some(x) => x,
            None => break,
        };
        let n = match prompt_and_then("Bits (dec): ", |line| line.trim().parse::<u32>().ok())? {
            Some(x) => x,
            None => break,
        };

        let results = booth3(a, b, n);
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
