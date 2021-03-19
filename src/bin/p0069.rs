use rayon::prelude::*;
// use reikna::totient::totient;

// too slow...
// pub fn main() {
//     let max_ = (1..1e4 as u64).into_par_iter().map(|n| (n, n / totient(n))).max_by(|x, y| x.1.cmp(&y.1));
//     println!("max_: {:#?}", max_);
// }

use reikna::totient::totient_all;
pub fn main() {
    let nums: Vec<u64> = (1..1_000_001).collect();
    let totients = totient_all(nums.clone());

    let ratios: Vec<_> = (0..1_000_000 as usize)
        .into_par_iter()
        .map(|i| nums[i] as f64 / totients[i] as f64)
        .collect();

    let mut max_ = 0.0f64;
    let mut num = 0;
    for (i, ratio) in ratios.iter().enumerate() {
        if *ratio > max_ {
            max_ = *ratio;
            num = nums[i];
        }
    }
    println!("max_: {:#?}", max_);
    println!("num: {:#?}", num);
}
