fn sum_of_squares(n: i64) -> i64 {
    n*(n+1)*(2*n+1)/6    
}

fn square_of_sum(n: i64) -> i64 {
    (n*(n+1)/2).pow(2)
}

fn main() {
    let limit = 100;
    println!("{}", (sum_of_squares(limit) - square_of_sum(limit)).abs());

}
