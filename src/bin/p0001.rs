#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn razor_sum(bound: u32, razors: &Vec<u32>) -> u32 {
    (1..bound).filter(|&n| {
        for r in razors.iter() {
            if n%r==0 {
                return true
            }
        }
        return false
    }).sum()
}

fn main() {
    let bound: u32 = 1000;
    let filters: Vec<u32> = vec![3,5];
    println!("{}", razor_sum(bound, &filters));
}
