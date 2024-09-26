use super::{
    deduction::{Deduction, DeductionRule},
    errors::Error,
    sequent::Sequent,
};

#[derive(Clone, PartialEq, Eq)]
pub struct Proof {
    conclusion: Sequent,
    premises: Vec<Proof>,
}

impl Proof {
    pub fn new(rule: DeductionRule) -> Proof {
        let premises = rule
            .get_premises()
            .into_iter()
            .map(|s| Proof {
                conclusion: s,
                premises: vec![],
            })
            .collect();
        Proof {
            conclusion: rule.get_conclusion(),
            premises,
        }
    }

    pub fn extend_bottom(&mut self, rule: DeductionRule) -> Result<(), Error> {
        let rule_premises = rule.get_premises();
        if rule_premises.len() != 1 {
            Err(Error::WrongNumberOfPremises {
                expected: 1,
                found: rule_premises.len() as i32,
            })
        } else {
            Ok(())
        }?;
        let premise = rule_premises
            .first()
            .ok_or(Error::MissingPremise(self.conclusion.clone()))?;
        if *premise == self.conclusion {
            Ok(())
        } else {
            Err(Error::SequentMismatch(
                self.conclusion.clone(),
                premise.to_owned(),
            ))
        }?;

        let new_premise = Proof {
            conclusion: self.conclusion.clone(),
            premises: self.premises.clone(),
        };
        self.premises = vec![new_premise];
        self.conclusion = rule.get_conclusion();
        Ok(())
    }

    pub fn combine_bottom(premises: Vec<Proof>, rule: DeductionRule) -> Result<Proof, Error> {
        let rule_premises = rule.get_premises();
        if premises.len() != rule_premises.len() {
            Err(Error::WrongNumberOfPremises {
                expected: rule_premises.len() as i32,
                found: premises.len() as i32,
            })
        } else {
            Ok(())
        }?;
        for (proof_premise, rule_premise) in premises.iter().zip(rule_premises.iter()) {
            if proof_premise.conclusion != *rule_premise {
                Err(Error::SequentMismatch(
                    proof_premise.conclusion.to_owned(),
                    rule_premise.to_owned(),
                ))
            } else {
                Ok(())
            }?;
        }

        let new_proof = Proof {
            premises,
            conclusion: rule.get_conclusion(),
        };
        Ok(new_proof)
    }
}
