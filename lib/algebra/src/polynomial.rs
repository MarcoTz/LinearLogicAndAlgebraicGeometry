use super::{function::BinOp, group::Group, monoid::Monoid, ring::Ring, set::Set};
use std::{collections::HashSet, marker::PhantomData};

pub type Variable = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariablePow {
    pub var: Variable,
    pub pow: i32,
}

#[derive(Debug, Clone)]
pub struct Monomial<C> {
    pub coefficient: C,
    pub vars: HashSet<VariablePow>,
}

#[derive(Debug, Clone)]
pub struct Polynomial<C> {
    pub monomials: Vec<Monomial<C>>,
}

pub struct PolynomialRing<Prod, Sum, R>
where
    R: Ring<Prod, Sum>,
    Prod: BinOp<R>,
    Sum: BinOp<R>,
{
    _base: R,
    _prod: PhantomData<Prod>,
    _sum: PhantomData<Sum>,
}

pub struct PolynomialProd;
pub struct PolynomialSum;

impl<R, Sum, Prod> BinOp<PolynomialRing<Prod, Sum, R>> for PolynomialProd
where
    R: Ring<Prod, Sum>,
    Prod: BinOp<R>,
    Sum: BinOp<R>,
{
    fn apply(
        &self,
        a: Polynomial<R::Element>,
        b: Polynomial<R::Element>,
    ) -> Polynomial<R::Element> {
        let mut new_monomials = vec![];
        for mon_a in a.monomials.iter() {
            for mon_b in b.monomials.iter() {
                let new_vars = mon_a.vars.union(&mon_b.vars).cloned().collect();
                let new_monomial = Monomial {
                    coefficient: <R as Ring<Prod, Sum>>::mult(
                        mon_a.coefficient.clone(),
                        mon_b.coefficient.clone(),
                    ),
                    vars: new_vars,
                };
                new_monomials.push(new_monomial);
            }
        }
        Polynomial {
            monomials: new_monomials,
        }
    }
}

impl<R, Prod, Sum> BinOp<PolynomialRing<Prod, Sum, R>> for PolynomialSum
where
    R: Ring<Prod, Sum>,
    Prod: BinOp<R>,
    Sum: BinOp<R>,
{
    fn apply(
        &self,
        a: Polynomial<R::Element>,
        b: Polynomial<R::Element>,
    ) -> Polynomial<R::Element> {
        let mut new_poly = a;
        for mon_b in b.monomials.iter() {
            match new_poly
                .monomials
                .iter_mut()
                .find(|mon_a| mon_a.vars == mon_b.vars)
            {
                None => new_poly.monomials.push(mon_b.clone()),
                Some(a) => {
                    a.coefficient = <R as Ring<Prod, Sum>>::add(
                        a.coefficient.clone(),
                        mon_b.coefficient.clone(),
                    )
                }
            }
        }

        new_poly
    }
}

impl<Prod, Sum, R> Set for PolynomialRing<Prod, Sum, R>
where
    R: Ring<Prod, Sum>,
    Prod: BinOp<R>,
    Sum: BinOp<R>,
{
    type Element = Polynomial<R::Element>;
}

impl<Prod, Sum, R> Monoid<PolynomialProd> for PolynomialRing<Prod, Sum, R>
where
    R: Ring<Prod, Sum>,
    Prod: BinOp<R>,
    Sum: BinOp<R>,
{
    fn prod(a: Polynomial<R::Element>, b: Polynomial<R::Element>) -> Polynomial<R::Element> {
        <PolynomialProd as BinOp<PolynomialRing<Prod, Sum, R>>>::apply(&PolynomialProd {}, a, b)
    }

    fn identity() -> Polynomial<R::Element> {
        Polynomial {
            monomials: vec![Monomial {
                coefficient: <R as Ring<Prod, Sum>>::one(),
                vars: HashSet::new(),
            }],
        }
    }
}

impl<R, Prod, Sum> Group<PolynomialSum> for PolynomialRing<Prod, Sum, R>
where
    R: Ring<Prod, Sum>,
    Prod: BinOp<R>,
    Sum: BinOp<R>,
{
    fn inverse(a: Polynomial<R::Element>) -> Polynomial<R::Element> {
        let mut new_poly = a;
        for mon in new_poly.monomials.iter_mut() {
            mon.coefficient = <R as Ring<Prod, Sum>>::neg(mon.coefficient.clone());
        }
        new_poly
    }
}

impl<Prod, Sum, R> Ring<PolynomialProd, PolynomialSum> for PolynomialRing<Prod, Sum, R>
where
    R: Ring<Prod, Sum>,
    Prod: BinOp<R>,
    Sum: BinOp<R>,
{
}
