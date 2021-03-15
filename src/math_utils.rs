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

