#![warn(
    bad_style,
    unused,
    unused_results
)]

fn main() {
    let mut largest: u64 = 1;
    let mut largest_len: u64 = 1;
    for q in 2..1000 {
        if q%2!=0 && q%5!=0 {
            let mut len: u64 = 1;
            let mut modulo: u64 = 10%q;
            while modulo != 1 {
                modulo = 10*modulo % q;
                len +=1;
            }
            if len > largest_len {
                largest_len = len;
                largest = q;
            }
        }
    }
    
    println!("largest denominator: {}, largest period: {}", largest, largest_len);
}
