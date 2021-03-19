extern crate marathon592;

use marathon592::math_utils::gcd_bigint;
use std::ops;
use primitive_types::U512;


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Fraction {
    numer: U512,
    denom: U512,
}

// From<U512> --------------------------------------------------------------------------------------

impl From<U512> for Fraction {
    fn from(val: U512) -> Self {
        Fraction {
            numer: val,
            denom: 1.into(),
        }
    }
}

// Add operator -----------------------------------------------------------------------------------

impl ops::Add<U512> for Fraction {
    type Output = Fraction;

    fn add(self, _rhs: U512) -> Fraction {
        self + Fraction::from(_rhs)
    }
}

impl ops::Add<Fraction> for U512 {
    type Output = Fraction;

    fn add(self, _rhs: Fraction) -> Fraction {
        // delegate
        _rhs + Fraction::from(self)
    }
}

impl ops::Add for Fraction {
    type Output = Fraction;

    fn add(self, _rhs: Fraction) -> Fraction {
        let numer = self.numer * _rhs.denom + _rhs.numer * self.denom;
        let denom = self.denom * _rhs.denom;
        return Fraction { numer, denom };
    }
}

impl Fraction {
    pub fn _onehalf() -> Self {
        Fraction {
            numer: 1.into(),
            denom: 2.into(),
        }
    }

    pub fn reverse(&self) -> Self {
        Fraction {
            numer: self.denom,
            denom: self.numer
        }
    }

    pub fn make_irreducible(&mut self) -> &Self {
        let gcd = gcd_bigint(&self.numer, &self.denom);

        if gcd != 1.into() {
            self.numer /= gcd;
            self.denom /= gcd;
        }

        self
    }
}

fn numer_has_more_digits(frac: &Fraction) -> bool {
    frac.numer.to_string().len() > frac.denom.to_string().len()
}

fn get_fraction(curr_fraction: &Fraction, curr_expansion: u32, limit_expansion: u32) -> Fraction {
    if curr_expansion == limit_expansion {
        let final_denom: Fraction = U512::from(2) + curr_fraction.reverse();
        return U512::from(1) + final_denom.reverse();
    }

    let curr_fraction = U512::from(2) + curr_fraction.reverse();
    get_fraction(&curr_fraction, curr_expansion + 1, limit_expansion)
}

pub fn main() {
    let mut sum = 0;
    for num_expansions in 8..1001 {
        let final_frac = get_fraction(&Fraction{numer: U512::from(2), denom: U512::from(1)}, 1, num_expansions - 1);
        if numer_has_more_digits(&final_frac) {
            sum = sum + 1;
            println!("Found #{} @{}: {:#?}", sum, num_expansions, final_frac);
        }

    }

}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn make_irreducible_test() {
        let input_to_expected_output: HashMap<Fraction, Fraction> = [
            (
                Fraction {
                    numer: 120,
                    denom: 90,
                },
                Fraction { numer: 4, denom: 3 },
            ),
            (
                Fraction {
                    numer: 180,
                    denom: 90,
                },
                Fraction { numer: 2, denom: 1 },
            ),
            (
                Fraction {
                    numer: 181,
                    denom: 90,
                },
                Fraction {
                    numer: 181,
                    denom: 90,
                },
            ),
        ]
        .iter()
        .cloned()
        .collect();

        for (input, expected_output) in &input_to_expected_output {
            assert_eq!(input.clone().make_irreducible(), expected_output);
        }
    }
}
