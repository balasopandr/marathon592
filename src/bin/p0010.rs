use marathon592::math_utils;

pub fn main() {
    let primes = math_utils::eratosthenes_sieve(2_000_000u64);
    let sum: u64 = primes.iter().sum();
    println!("sum: {:#?}", sum);
}


