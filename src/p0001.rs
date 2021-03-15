#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn razor_sum(bound: u32, razors: [u32;2]) -> u32 {
    (1..bound).filter(|n| n%3==0 || n%5 ==0).sum()
    //(1..bound).filter(|n| razors.into_iter().filter(|r| n%r==0)).sum()
}

fn main() {
    let bound: u32 = 1000;
    let filters: [u32;2] = [3,5];
    println!("{}", razor_sum(bound, filters));
}
