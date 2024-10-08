use std::{
    fmt,
    ops::{Add, Mul},
};

#[derive(Clone)]
pub struct Monomial<C> {
    pub coefficient: C,
    pub powers: Vec<i32>,
}

impl<C> Monomial<C> {
    fn same_powers(&self, other: &Monomial<C>) -> bool {
        if self.powers.len() != other.powers.len() {
            return false;
        }
        self.powers
            .iter()
            .zip(other.powers.iter())
            .fold(true, |same, (self_power, other_power)| {
                same && self_power == other_power
            })
    }
}

pub struct Polynomial<C> {
    pub monomials: Vec<Monomial<C>>,
}

impl<C: fmt::Display> fmt::Display for Monomial<C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let var_str: Vec<String> = self
            .powers
            .iter()
            .enumerate()
            .map(|(index, power)| format!("X_{}^{}", index, power))
            .collect();
        write!(f, "{}{}", self.coefficient, var_str.join(","))
    }
}

impl<C: fmt::Display> fmt::Display for Polynomial<C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mon_str: Vec<String> = self
            .monomials
            .iter()
            .map(|mon| format!("{}", mon))
            .collect();
        write!(f, "{}", mon_str.join(" + "))
    }
}

impl<C: Add<Output = C>> Add for Monomial<C> {
    type Output = Polynomial<C>;
    fn add(self, other: Self) -> Self::Output {
        if self.same_powers(&other) {
            Polynomial {
                monomials: vec![Monomial {
                    coefficient: self.coefficient + other.coefficient,
                    powers: self.powers,
                }],
            }
        } else {
            Polynomial {
                monomials: vec![self, other],
            }
        }
    }
}

impl<C> Add for Polynomial<C>
where
    C: Add<Output = C>,
{
    type Output = Polynomial<C>;
    fn add(self, other: Self) -> Self::Output {
        let mut new_monomials = self.monomials;
        for other_mono in other.monomials.into_iter() {
            match new_monomials
                .iter()
                .enumerate()
                .find(|(_, mono)| mono.same_powers(&other_mono))
            {
                None => {
                    new_monomials.push(other_mono);
                }
                Some((ind, _)) => {
                    let old_mono = new_monomials.remove(ind);
                    let new_poly = old_mono + other_mono;
                    new_monomials.extend(new_poly.monomials);
                }
            }
        }
        Polynomial {
            monomials: new_monomials,
        }
    }
}

impl<C> Mul for Monomial<C>
where
    C: Add<Output = C>,
    C: Mul,
{
    type Output = Monomial<<C as Mul>::Output>;
    fn mul(self, other: Self) -> Self::Output {
        let mut new_powers = self.powers;
        for (ind, pow) in other.powers.iter().enumerate() {
            match new_powers.get(ind) {
                None => new_powers.push(*pow),
                Some(power) => new_powers.insert(ind, power * pow),
            }
        }
        Monomial {
            coefficient: self.coefficient * other.coefficient,
            powers: new_powers,
        }
    }
}
impl<C> Mul for Polynomial<C>
where
    C: Add<Output = C>,
    C: Mul<Output = C>,
    C: Clone,
{
    type Output = Polynomial<C>;
    fn mul(self, other: Self) -> Self::Output {
        let mut new_monomials: Vec<Monomial<C>> = vec![];
        for self_mono in self.monomials.iter() {
            for other_mono in other.monomials.iter() {
                let mut new_mono = self_mono.clone() * other_mono.clone();
                if let Some(mono) = new_monomials
                    .iter()
                    .find(|mono| mono.same_powers(&new_mono))
                {
                    new_mono.coefficient = new_mono.coefficient + mono.coefficient.clone();
                }
                new_monomials.push(new_mono);
            }
        }
        Polynomial {
            monomials: new_monomials,
        }
    }
}
