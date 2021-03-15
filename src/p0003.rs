#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use math::is_prime;

fn main() {
    let mut i: u64 = 2;
    let num: u64 = 600851475143;
    let mut biggest_prime: u64 = 1;
    while i*i< num {
        if num%i == 0 && is_prime(i) {
            biggest_prime = i;
        }
        i += 1
    }
    println!("{}", biggest_prime);
}
