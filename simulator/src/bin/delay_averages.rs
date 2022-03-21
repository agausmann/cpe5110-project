use cpe5110_project::{booth3, booth4, Results};

const BITS: [u32; 5] = [4, 6, 8, 10, 12];
const GROUP_SIZES: [(u32, fn(i32, i32, u32) -> Results); 2] = [(3, booth3), (4, booth4)];

fn main() {
    println!("Bits,Average delay (Booth3),Average delay (Booth4)");
    for bits in BITS {
        print!("{}", bits);
        for (_, booth_fn) in GROUP_SIZES {
            let mut delays = Vec::with_capacity(2usize.pow(bits));
            for a in -2i32.pow(bits - 1)..2i32.pow(bits - 1) {
                for b in -2i32.pow(bits - 1)..2i32.pow(bits - 1) {
                    delays.push(booth_fn(a, b, bits).delay);
                }
            }
            let n = delays.len();
            print!(",{}", (delays.into_iter().sum::<u32>() as f32) / (n as f32));
        }
        println!();
    }
}
