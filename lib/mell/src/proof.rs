use super::{
    errors::Error,
    formula::Formula,
    sequent::{split_seq, Sequent},
};
use std::{fmt, rc::Rc};

#[derive(Debug)]
pub enum Rule {
    Ax,
    Cut,
    Tensor,
    Par,
    Ex,
    Der,
    Prom,
    Weak,
    Ctr,
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rule::Ax => f.write_str("Ax"),
            Rule::Cut => f.write_str("Cut"),
            Rule::Tensor => f.write_str("Tensor"),
            Rule::Par => f.write_str("Par"),
            Rule::Ex => f.write_str("Ex"),
            Rule::Der => f.write_str("Der"),
            Rule::Prom => f.write_str("Prom"),
            Rule::Weak => f.write_str("Weak"),
            Rule::Ctr => f.write_str("Ctr"),
        }
    }
}

pub struct Proof {
    pub premises: Vec<Proof>,
    pub rule: Rule,
    pub conclusion: Sequent,
}

pub fn ax(f: Formula) -> Proof {
    Proof {
        premises: vec![],
        rule: Rule::Ax,
        conclusion: vec![f.clone().neg(), f],
    }
}

pub fn cut(left: Proof, right: Proof, active: &Formula) -> Result<Proof, Error> {
    let active_neg = active.clone().neg();
    let split_left = split_seq(left.conclusion.clone(), active)?;
    let split_right = split_seq(right.conclusion.clone(), &active_neg)?;
    let mut conclusion = split_left.left;
    conclusion.extend(split_left.right);
    conclusion.extend(split_right.left);
    conclusion.extend(split_right.right);
    Ok(Proof {
        premises: vec![left, right],
        rule: Rule::Cut,
        conclusion,
    })
}

pub fn tensor(
    left: Proof,
    right: Proof,
    active_a: &Formula,
    active_b: &Formula,
) -> Result<Proof, Error> {
    let split_left = split_seq(left.conclusion.clone(), active_a)?;
    let split_right = split_seq(right.conclusion.clone(), active_b)?;
    let mut conclusion = split_left.left;
    conclusion.extend(split_left.right);
    conclusion.push(Formula::Tensor(
        Rc::new(split_left.active),
        Rc::new(split_right.active),
    ));
    conclusion.extend(split_right.left);
    conclusion.extend(split_right.right);

    Ok(Proof {
        premises: vec![left, right],
        rule: Rule::Tensor,
        conclusion,
    })
}

pub fn par(prem: Proof, active_a: &Formula, active_b: &Formula) -> Result<Proof, Error> {
    let split1 = split_seq(prem.conclusion.clone(), active_a)?;
    let mut new_s = split1.left;
    new_s.extend(split1.right);
    let split2 = split_seq(new_s, active_b)?;
    let mut conclusion = split2.left;
    conclusion.push(Formula::Par(Rc::new(split1.active), Rc::new(split2.active)));
    conclusion.extend(split2.right);
    Ok(Proof {
        premises: vec![prem],
        rule: Rule::Par,
        conclusion,
    })
}

pub fn ex(prem: Proof, active_a: &Formula, active_b: &Formula) -> Result<Proof, Error> {
    let split1 = split_seq(prem.conclusion.clone(), active_a)?;
    let mut new_s = split1.left;
    new_s.extend(split1.right);
    let split2 = split_seq(new_s, active_b)?;
    let mut conclusion = split2.left;
    conclusion.push(split2.active);
    conclusion.push(split1.active);
    conclusion.extend(split2.right);

    Ok(Proof {
        premises: vec![prem],
        rule: Rule::Ex,
        conclusion,
    })
}

pub fn der(prem: Proof, active: &Formula) -> Result<Proof, Error> {
    let split = split_seq(prem.conclusion.clone(), active)?;
    let mut conclusion = split.left;
    conclusion.push(Formula::Quest(Rc::new(split.active)));
    conclusion.extend(split.right);
    Ok(Proof {
        premises: vec![prem],
        rule: Rule::Der,
        conclusion,
    })
}

pub fn prom(prem: Proof, active: &Formula) -> Result<Proof, Error> {
    let non_quest: Vec<&Formula> = prem
        .conclusion
        .iter()
        .filter(|f| !matches!(f, Formula::Quest(_)))
        .collect();
    if non_quest.is_empty() {
        let split = split_seq(prem.conclusion.clone(), active)?;
        let mut conclusion = split.left;
        conclusion.push(Formula::Bang(Rc::new(split.active)));
        conclusion.extend(split.right);
        Ok(Proof {
            premises: vec![prem],
            rule: Rule::Prom,
            conclusion,
        })
    } else {
        Err(Error::CannotApply(Rule::Prom))
    }
}

pub fn weak(prem: Proof, active: &Formula) -> Proof {
    let mut conclusion = prem.conclusion.clone();
    conclusion.push(Formula::Quest(Rc::new(active.to_owned())));
    Proof {
        premises: vec![prem],
        rule: Rule::Weak,
        conclusion,
    }
}

pub fn ctr(prem: Proof, active: &Formula) -> Result<Proof, Error> {
    if matches!(active, Formula::Quest(_)) {
        let split1 = split_seq(prem.conclusion.clone(), active)?;
        let mut new_s = split1.left;
        new_s.extend(split1.right);
        let split2 = split_seq(new_s, active)?;
        let mut conclusion = split2.left;
        conclusion.push(split2.active);
        conclusion.extend(split2.right);
        Ok(Proof {
            premises: vec![prem],
            rule: Rule::Ctr,
            conclusion,
        })
    } else {
        Err(Error::CannotApply(Rule::Ctr))
    }
}

#[cfg(test)]
mod proof_test {
    use super::{ax, ctr, der, tensor};
    use crate::{
        formula::Formula,
        preformula::{OrientedAtom, Polarity},
    };
    use std::rc::Rc;

    #[test]
    fn church_2() {
        let atom: Formula = OrientedAtom {
            atom: "A".to_owned(),
            pol: Polarity::Pos,
        }
        .into();
        let atom_neg = atom.clone().neg();
        let ax_1 = ax(atom.clone());
        let ax_2 = ax(atom.clone());
        let tensor_1 = tensor(ax_1, ax_2, &atom, &atom_neg).unwrap();
        let ax_3 = ax(atom.clone());
        let tensor_2 = tensor(ax_3, tensor_1, &atom, &atom_neg).unwrap();
        let active = Formula::Tensor(Rc::new(atom.clone()), Rc::new(atom_neg.clone()));
        let der_1 = der(tensor_2, &active).unwrap();
        let der_2 = der(der_1, &active).unwrap();
        ctr(der_2, &Formula::Quest(Rc::new(active))).unwrap();
    }
}
