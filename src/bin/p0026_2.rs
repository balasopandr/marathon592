pub fn main() {
    let nums = 2..1000;
    let denom_cycle_pair = nums.map(find_length_of).max_by(|x, y| x.1.cmp(&y.1));
    println!(
        "Number and corresponding cycle: {:#?}",
        denom_cycle_pair.unwrap()
    );
}

fn find_length_of(denom: u32) -> (u32, u32) {
    // skip factors of 2 and 5
    if denom % 2 == 0 || denom % 5 == 0 {
        return (denom, 0);
    }

    let mut n = 1;
    let mut mod_ = 10u32 % denom;

    while mod_ != 1 {
        mod_ = (mod_ * 10) % denom;
        n += 1;

        if n > 1000 {
            panic!("Something's wrong, too large of a number reached")
        }
    }

    return (denom, n);
}
