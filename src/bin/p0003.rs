// TODO Probably there's a way to refer to the root of this package. Can't seem to reach it with
// `crate` or `super`
use marathon592::math_utils;

fn main() {
    let mut i: u64 = 2;
    let num: u64 = 600851475143;
    let mut biggest_prime: u64 = 1;
    while i * i < num {
        if num % i == 0 && math_utils::is_prime(i) {
            biggest_prime = i;
        }
        i += 1
    }
    println!("{}", biggest_prime);
}
