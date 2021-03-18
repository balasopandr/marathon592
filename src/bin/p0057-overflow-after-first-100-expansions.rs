extern crate marathon592;

use marathon592::math_utils::gcd;
use std::ops;


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Fraction {
    numer: u128,
    denom: u128,
}

// From<u128> --------------------------------------------------------------------------------------

impl From<u128> for Fraction {
    fn from(val: u128) -> Self {
        Fraction {
            numer: val,
            denom: 1,
        }
    }
}

// Add operator -----------------------------------------------------------------------------------

impl ops::Add<u128> for Fraction {
    type Output = Fraction;

    fn add(self, _rhs: u128) -> Fraction {
        self + Fraction::from(_rhs)
    }
}

impl ops::Add<Fraction> for u128 {
    type Output = Fraction;

    fn add(self, _rhs: Fraction) -> Fraction {
        // delegate
        _rhs + Fraction::from(self)
    }
}

impl ops::Add<Fraction> for Fraction {
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
            numer: 1,
            denom: 2,
        }
    }

    pub fn reverse(&self) -> Self {
        Fraction {
            numer: self.denom,
            denom: self.numer
        }
    }

    pub fn make_irreducible(&mut self) -> &Self {
        let gcd = gcd::<u128>(self.numer, self.denom);

        if gcd != 1 {
            self.numer /= gcd;
            self.denom /= gcd;
        }

        self
    }
}

fn numer_has_more_digits(frac: &Fraction) -> bool {
    frac.numer.to_string().len() > frac.denom.to_string().len()
}

fn get_fraction(curr_fraction: &Fraction, curr_expansion: u128, limit_expansion: u128) -> Fraction {
    if curr_expansion == limit_expansion {
        let final_denom = 2 + curr_fraction.reverse();
        return 1 + final_denom.reverse();
    }

    let curr_fraction = 2 + curr_fraction.reverse();
    get_fraction(&curr_fraction, curr_expansion + 1, limit_expansion)
}

pub fn main() {
    let mut sum = 0;
    for num_expansions in 8..1001 {
        let final_frac = get_fraction(&Fraction{numer: 2, denom: 1}, 1, num_expansions - 1);
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
