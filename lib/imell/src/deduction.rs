use super::{
    errors::Error,
    formula::{split_seq, Formula, Sequent},
};
use std::{fmt, rc::Rc};

#[derive(Debug, Clone)]
pub enum Rule {
    Ax,
    Cut,
    LTensor,
    RTensor,
    LImpl,
    RImpl,
    Ex,
    Der,
    Prom,
    Weak,
    Ctr,
}

pub struct Proof {
    premises: Vec<Sequent>,
    rule: Rule,
    conclusion: Sequent,
}

pub fn ax(f: &Formula) -> Proof {
    Proof {
        premises: vec![],
        rule: Rule::Ax,
        conclusion: Sequent {
            antecent: vec![f.to_owned()],
            succedent: f.to_owned(),
        },
    }
}

pub fn cut(left: Sequent, right: Sequent) -> Result<Proof, Error> {
    let a = left.succedent.clone();
    let active_err = Error::ActiveNotFound(a.clone(), right.clone());
    let b = right.succedent.clone();
    let mut new_right = right.antecent.clone();
    let a2 = new_right.pop().ok_or(active_err.clone())?;
    if a == a2 {
        let mut new_ante = left.antecent.clone();
        new_ante.extend(new_right);
        Ok(Proof {
            premises: vec![left, right],
            rule: Rule::Cut,
            conclusion: Sequent {
                antecent: new_ante,
                succedent: b,
            },
        })
    } else {
        Err(active_err)
    }
}

pub fn left_tensor(prem: Sequent, active_a: &Formula, active_b: &Formula) -> Result<Proof, Error> {
    let split_a = split_seq(active_a, prem.clone())?;
    let split_b = split_seq(
        active_b,
        Sequent {
            antecent: split_a.right_ante,
            succedent: split_a.succedent,
        },
    )?;
    let mut new_ante = split_a.left_ante;
    new_ante.extend(split_b.left_ante);
    new_ante.extend(split_b.right_ante);
    Ok(Proof {
        premises: vec![prem],
        rule: Rule::LTensor,
        conclusion: Sequent {
            antecent: new_ante,
            succedent: split_b.succedent,
        },
    })
}

pub fn right_tensor(left: Sequent, right: Sequent) -> Proof {
    let mut antecent = left.antecent.clone();
    antecent.extend(right.antecent.clone());
    let succedent = Formula::Tensor(
        Rc::new(left.succedent.clone()),
        Rc::new(right.succedent.clone()),
    );
    Proof {
        premises: vec![left, right],
        rule: Rule::RTensor,
        conclusion: Sequent {
            antecent,
            succedent,
        },
    }
}

pub fn left_impl(left: Sequent, right: Sequent, active_b: &Formula) -> Result<Proof, Error> {
    let split = split_seq(active_b, right.clone())?;
    let mut antecent = left.antecent.clone();
    antecent.push(Formula::Impl(
        Rc::new(left.succedent.clone()),
        Rc::new(split.active),
    ));
    antecent.extend(split.left_ante);
    antecent.extend(split.right_ante);
    let succedent = right.succedent.clone();
    Ok(Proof {
        premises: vec![left, right],
        rule: Rule::LImpl,
        conclusion: Sequent {
            antecent,
            succedent,
        },
    })
}

pub fn right_impl(prem: Sequent, active_a: &Formula) -> Result<Proof, Error> {
    let split = split_seq(active_a, prem.clone())?;
    let mut antecent = split.left_ante;
    antecent.extend(split.right_ante);
    let succedent = Formula::Impl(Rc::new(split.active), Rc::new(prem.succedent.clone()));
    Ok(Proof {
        premises: vec![prem],
        rule: Rule::RImpl,
        conclusion: Sequent {
            antecent,
            succedent,
        },
    })
}

pub fn ex(prem: Sequent, active_a: &Formula, active_b: &Formula) -> Result<Proof, Error> {
    let split_a = split_seq(active_a, prem.clone())?;
    let split_b = split_seq(
        active_b,
        Sequent {
            antecent: split_a.right_ante,
            succedent: split_a.succedent,
        },
    )?;
    let mut antecent = split_a.left_ante;
    antecent.push(split_b.active);
    antecent.push(split_a.active);
    antecent.extend(split_b.left_ante);
    antecent.extend(split_b.right_ante);
    Ok(Proof {
        premises: vec![prem],
        rule: Rule::Ex,
        conclusion: Sequent {
            antecent,
            succedent: split_b.succedent,
        },
    })
}

pub fn der(prem: Sequent, active: &Formula) -> Result<Proof, Error> {
    let split = split_seq(active, prem.clone())?;
    let mut antecent = split.left_ante;
    antecent.push(Formula::Bang(Rc::new(split.active)));
    antecent.extend(split.right_ante);
    Ok(Proof {
        premises: vec![prem],
        rule: Rule::Der,
        conclusion: Sequent {
            antecent,
            succedent: split.succedent,
        },
    })
}

pub fn prom(prem: Sequent) -> Result<Proof, Error> {
    let non_bang: Vec<&Formula> = prem
        .antecent
        .iter()
        .filter(|form| matches!(form, Formula::Bang(_)))
        .collect();
    if non_bang.is_empty() {
        Ok(Proof {
            premises: vec![prem.clone()],
            rule: Rule::Prom,
            conclusion: Sequent {
                antecent: prem.antecent,
                succedent: Formula::Bang(Rc::new(prem.succedent)),
            },
        })
    } else {
        Err(Error::CannotApply(Rule::Prom))
    }
}

pub fn weak(prem: Sequent, active: &Formula) -> Proof {
    let mut antecent = prem.antecent.clone();
    antecent.push(Formula::Bang(Rc::new(active.to_owned())));
    let succedent = prem.succedent.clone();

    Proof {
        premises: vec![prem],
        rule: Rule::Weak,
        conclusion: Sequent {
            antecent,
            succedent,
        },
    }
}

pub fn ctr(prem: Sequent, active_a: &Formula) -> Result<Proof, Error> {
    if matches!(active_a, Formula::Bang(_)) {
        Ok(())
    } else {
        Err(Error::CannotApply(Rule::Ctr))
    }?;

    let split_1 = split_seq(active_a, prem.clone())?;
    let split_2 = split_seq(
        active_a,
        Sequent {
            antecent: split_1.right_ante,
            succedent: split_1.succedent,
        },
    )?;
    let mut antecent = split_1.left_ante;
    antecent.push(split_1.active);
    antecent.extend(split_2.left_ante);
    antecent.extend(split_2.right_ante);
    Ok(Proof {
        premises: vec![prem],
        rule: Rule::Ctr,
        conclusion: Sequent {
            antecent,
            succedent: split_2.succedent,
        },
    })
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rule::Ax => f.write_str("Ax"),
            Rule::Cut => f.write_str("Cut"),
            Rule::LTensor => f.write_str("LTensor"),
            Rule::RTensor => f.write_str("RTensor"),
            Rule::LImpl => f.write_str("LImpl"),
            Rule::RImpl => f.write_str("RImpl"),
            Rule::Ex => f.write_str("Ex"),
            Rule::Der => f.write_str("Def"),
            Rule::Prom => f.write_str("Prom"),
            Rule::Weak => f.write_str("Weak"),
            Rule::Ctr => f.write_str("Ctr"),
        }
    }
}
