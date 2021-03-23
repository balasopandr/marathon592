use marathon592::math_utils;
fn main() {
    let mut i = 1;
    let mut primes = 0;
    let mut last_prime = 0;
    let limit = 10001;

    // if it looks stupid but it works, it ain't stupid
    while primes < limit {
        i += 1;
        if math_utils::is_prime(i) {
            primes +=1;
            last_prime = i;
        }
    }

    println!("{}", last_prime);

}
