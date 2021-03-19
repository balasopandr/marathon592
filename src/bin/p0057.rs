use num_bigint::ToBigUint;

// Algorithm shamelessly stolen from
// https://radiusofcircle.blogspot.com/2016/06/problem-57-project-euler-solution-with-python.html
pub fn main() {
    let mut p = 1.to_biguint().unwrap();
    let mut q = 1.to_biguint().unwrap();

    let mut cnt = 0;

    for _ in 0..1000 {
        let a1 = p.clone() + 2.to_biguint().unwrap() * q.clone();
        let b1 = p + q;

        if a1.to_string().len() > b1.to_string().len() {
            cnt = cnt + 1;
        }

        p = a1;
        q = b1;
    }

println!("cnt: {:#?}", cnt);
}
