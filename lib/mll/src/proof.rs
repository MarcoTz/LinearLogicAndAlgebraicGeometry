use super::{
    deduction::{Ax, Deduction, DeductionRule},
    errors::Error,
};

#[derive(Clone, PartialEq, Eq)]
pub struct Proof {
    conclusion: DeductionRule,
    premises: Vec<Proof>,
}

impl Proof {
    pub fn new(ax: Ax) -> Proof {
        Proof {
            premises: vec![],
            conclusion: ax.into(),
        }
    }

    pub fn conclusion(&self) -> DeductionRule {
        self.conclusion.to_owned()
    }

    pub fn premises(&self) -> Vec<Proof> {
        self.premises.to_owned()
    }

    pub fn combine(rule: DeductionRule, premises: Vec<Proof>) -> Result<Proof, Error> {
        let rule_premises = rule.get_premises();
        if premises.len() != rule_premises.len() {
            Err(Error::WrongNumberOfPremises {
                found: premises.len() as i32,
                expected: rule_premises.len() as i32,
            })
        } else {
            Ok(())
        }?;
        for (rule_premise, proof_premise) in rule_premises.iter().zip(premises.iter()) {
            let proof_conclusion = proof_premise.conclusion.get_conclusion();
            if proof_conclusion != *rule_premise {
                Err(Error::SequentMismatch(
                    proof_conclusion,
                    rule_premise.to_owned(),
                ))
            } else {
                Ok(())
            }?;
        }
        Ok(Proof {
            conclusion: rule,
            premises,
        })
    }
}
