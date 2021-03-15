#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn main() {
    let mut sum: u64 = 2;
    let mut fibs: Vec<u64> = vec![1,2];
    let mut idx: usize = 2;
    while fibs[idx-1] <= 4000000 {
        let a: u64 = fibs[(idx-1)/2];
        let b: u64 = fibs[(idx-1)/2 + 1];
        let c: u64 = a*(b*2 - a);
        let d: u64 = a*a + b*b;
        if idx%2 == 0 {
            fibs.push(c);
            fibs.push(d);
        }
        else {
            fibs.push(d);
            fibs.push(c+d);
        }
        let cur: u64 = fibs[idx];
        let next: u64 = fibs[idx+1];
        if cur%2 == 0 {
            sum += cur;
        }

        if next%2==0 {
            sum += next;
        }
        idx+=2;
    }
    println!("{}", sum);
}
