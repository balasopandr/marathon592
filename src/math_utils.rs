use num::{cast::AsPrimitive, ToPrimitive, Unsigned};
use std::vec::Vec;

pub fn is_prime(n: u64) -> bool {
    if n <= 3 {
        return n > 1;
    }

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut count = 5;

    while count * count <= n {
        if n % count == 0 || n % (count + 2) == 0 {
            return false;
        }
        count += 6;
    }
    return true;
}

/// Compute all the prime numbers up to the given number
///
/// # Examples
///
/// ```
/// use marathon592::math_utils;
/// let primes = math_utils::eratosthenes_sieve(10);
/// assert_eq!(primes.len(), 4);
/// assert_eq!(primes.iter().sum::<u32>(), 17);
///
/// let primes = math_utils::eratosthenes_sieve(2_000_000u64);
/// assert_eq!(primes.iter().sum::<u64>(), 142913828922);
/// ```
pub fn eratosthenes_sieve<T: 'static>(up_to: T) -> Vec<T>
where
    usize: AsPrimitive<T>,
    T: Unsigned + ToPrimitive + Copy + std::cmp::PartialOrd,
{
    // false == unchecked
    // true  == not prime
    let mut numbers_marked = vec![false; up_to.to_usize().unwrap() + 1];

    // 0 and 1 are not primes
    numbers_marked[0] = true;
    numbers_marked[1] = true;

    let mut i = 2;
    while i.as_() <= up_to {
        // already marked, thus not a prime, skip it.
        if numbers_marked[i as usize] {
            i += 1;
            continue;
        }

        // mark all the multiples
        let mut j = i + i;
        while j.as_() <= up_to {
            numbers_marked[j as usize] = true;
            j += i;
        }

        i += 1;
    }

    // return the indices, which are marked as false.
    let out = numbers_marked
        .iter()
        .enumerate()
        .filter(|pair| *pair.1 == false)
        .map(|x| x.0.as_())
        .collect::<Vec<T>>();

    return out;
}
