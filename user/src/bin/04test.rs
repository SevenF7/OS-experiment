#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

const LEN: usize = 50;

#[no_mangle]
fn main() -> i32 {
    let base = 2u64;
    let modulus = 1000000007u64;
    let iterations: usize = 150000;
    let mut sequence = [0u64; LEN];
    let mut current = 0usize;
    sequence[current] = 1;

    for i in 1..=iterations {
        let next = if current + 1 == LEN { 0 } else { current + 1 };
        sequence[next] = (sequence[current] * base) % modulus;
        current = next;

        if i % 25000 == 0 {
            println!("power_2 [{}/{}]", i, iterations);
        }
    }

    println!("{}^{} mod {} = {}", base, iterations, modulus, sequence[current]);
    println!("Test OK!");

    0
}

