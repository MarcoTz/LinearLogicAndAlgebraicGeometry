use super::{directed_graph::DirectedMultiGraph, errors::Error};
use std::fmt;

#[derive(Clone)]
pub enum RuleLabel {
    Ax,
    Cut,
    Tensor,
    Par,
    Bang,
    Quest,
    Ctr,
    Weak,
    Pax,
    Prom,
    Der,
    C,
}

pub struct Link<T> {
    premises: Vec<T>,
    rule: RuleLabel,
    conclusions: Vec<T>,
}

pub trait ProofStructure: DirectedMultiGraph {
    fn negate(e: Self::EdgeLabel) -> Self::EdgeLabel;
    fn tensor(e1: Self::EdgeLabel, e2: Self::EdgeLabel) -> Self::EdgeLabel;
    fn par(e1: Self::EdgeLabel, e2: Self::EdgeLabel) -> Self::EdgeLabel;
    fn is_quest(e: &Self::EdgeLabel) -> bool;
    fn add_link(&mut self, lnk: Link<Self::EdgeLabel>);

    fn ax_link(&mut self, conclusion: Self::EdgeLabel) {
        let conc_neg = Self::negate(conclusion.clone());
        let link = Link {
            premises: vec![],
            rule: RuleLabel::Ax,
            conclusions: vec![conclusion, conc_neg],
        };
        self.add_link(link)
    }

    fn cut_link(&mut self, premise: Self::EdgeLabel) {
        let premise_neg = Self::negate(premise.clone());
        let link = Link {
            premises: vec![premise, premise_neg],
            rule: RuleLabel::Cut,
            conclusions: vec![],
        };
        self.add_link(link)
    }

    fn tensor_link(&mut self, premise_left: Self::EdgeLabel, premise_right: Self::EdgeLabel) {
        let link = Link {
            premises: vec![premise_left.clone(), premise_right.clone()],
            rule: RuleLabel::Tensor,
            conclusions: vec![Self::tensor(premise_left, premise_right)],
        };
        self.add_link(link)
    }

    fn par_link(&mut self, premise_left: Self::EdgeLabel, premise_right: Self::EdgeLabel) {
        let link = Link {
            premises: vec![premise_left.clone(), premise_right.clone()],
            rule: RuleLabel::Par,
            conclusions: vec![Self::par(premise_left, premise_right)],
        };
        self.add_link(link)
    }

    fn ctr_link(&mut self, premise: Self::EdgeLabel) -> Result<(), Box<dyn std::error::Error>> {
        if !Self::is_quest(&premise) {
            Err(Error::BadLabel(format!("{}", premise)))
        } else {
            Ok(())
        }?;
        let link = Link {
            premises: vec![premise.clone(), premise.clone()],
            rule: RuleLabel::Ctr,
            conclusions: vec![premise],
        };
        self.add_link(link);
        Ok(())
    }

    fn pax_link(&mut self, premise: Self::EdgeLabel) -> Result<(), Box<dyn std::error::Error>> {
        if !Self::is_quest(&premise) {
            Err(Error::BadLabel(format!("{}", premise)))
        } else {
            Ok(())
        }?;
        let link = Link {
            premises: vec![premise.clone()],
            rule: RuleLabel::Pax,
            conclusions: vec![premise],
        };
        self.add_link(link);
        Ok(())
    }

    fn prom_link(&mut self, premise: Self::EdgeLabel) {
        let link = Link {
            premises: vec![premise.clone()],
            rule: RuleLabel::Prom,
            conclusions: vec![premise],
        };
        self.add_link(link)
    }

    fn weak_link(&mut self, conclusion: Self::EdgeLabel) {
        let link = Link {
            premises: vec![],
            rule: RuleLabel::Weak,
            conclusions: vec![conclusion],
        };
        self.add_link(link)
    }

    fn der_link(&mut self, premise: Self::EdgeLabel) {
        let link = Link {
            premises: vec![premise.clone()],
            rule: RuleLabel::Der,
            conclusions: vec![premise.clone()],
        };
        self.add_link(link)
    }

    fn conc_link(&mut self, premise: Self::EdgeLabel) {
        let link = Link {
            premises: vec![premise],
            rule: RuleLabel::C,
            conclusions: vec![],
        };
        self.add_link(link)
    }
}
