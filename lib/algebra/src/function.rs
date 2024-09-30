use super::set::{Set, Subset};

pub trait Function<Domain, Codomain>
where
    Domain: Set,
    Codomain: Set,
{
    fn apply(&self, x: Domain::Element) -> Codomain::Element;
}

pub trait BinOp<Domain>
where
    Domain: Set,
{
    fn apply(&self, x: Domain::Element, y: Domain::Element) -> Domain::Element;
}

impl<Domain, Codomain> Function<Domain, Codomain>
    for Box<dyn Fn(Domain::Element) -> Codomain::Element>
where
    Domain: Set,
    Codomain: Set,
{
    fn apply(&self, x: Domain::Element) -> Codomain::Element {
        self(x)
    }
}

pub fn restrict<'a, Domain, Codomain, Subdomain>(
    fun: impl Function<Domain, Codomain> + 'a,
) -> Box<dyn Fn(Subdomain::Element) -> Codomain::Element + 'a>
where
    Domain: Set,
    Codomain: Set,
    Subdomain: Subset<Domain>,
{
    Box::new(move |x: Subdomain::Element| fun.apply(<Subdomain as Subset<Domain>>::embed(x)))
}

impl<Domain> BinOp<Domain> for Box<dyn Fn((Domain::Element, Domain::Element)) -> Domain::Element>
where
    Domain: Set,
{
    fn apply(&self, x: Domain::Element, y: Domain::Element) -> Domain::Element {
        self((x, y))
    }
}
