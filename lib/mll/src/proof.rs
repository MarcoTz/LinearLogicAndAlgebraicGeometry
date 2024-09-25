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
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rule::Ax => f.write_str("Ax"),
            Rule::Cut => f.write_str("Cut"),
            Rule::Tensor => f.write_str("Tensor"),
            Rule::Par => f.write_str("Par"),
            Rule::Ex => f.write_str("Ex"),
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
