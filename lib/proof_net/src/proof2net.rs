use super::{
    errors::Error,
    links::{AttachContext, AttachToNet, AxLink, CutLink, ParLink, TensorLink},
    proof_structure::{ProofStructure, VertexLabel},
};
use mll::{
    deduction::{Deduction, DeductionRule},
    proof::Proof,
};

impl TryFrom<Proof> for ProofStructure {
    type Error = Error;
    fn try_from(proof: Proof) -> Result<ProofStructure, Error> {
        match proof.conclusion() {
            DeductionRule::Ax(ax) => {
                let context = AttachContext::default();
                let form = ax
                    .get_active()
                    .first()
                    .cloned()
                    .ok_or(Error::MissingConclusion)?;
                let mut net = ProofStructure::new();
                AxLink::new(form).attach(&mut net, context)?;
                Ok(net)
            }
            DeductionRule::Cut(cut) => {
                let cut_premises = proof.premises();
                if cut_premises.len() != 2 {
                    Err(Error::BadProof)
                } else {
                    Ok(())
                }?;
                let left_premise = cut_premises.first().unwrap().to_owned();
                let right_premise = cut_premises.get(1).unwrap().to_owned();

                let proof_left: ProofStructure = left_premise.try_into()?;
                let proof_right: ProofStructure = right_premise.try_into()?;

                let mut new_net = proof_left;
                new_net.disjoint_union(proof_right);
                let active = cut.get_active();
                let active_left = active.first().ok_or(Error::MissingPremise)?;
                let active_right = active.get(1).ok_or(Error::MissingPremise)?;
                let prev_left = new_net.find_conclusion(&active_left);
                let prev_right = new_net.find_conclusion(&active_right);

                let context = AttachContext {
                    prev_left: prev_left.as_ref(),
                    prev_right: prev_right.as_ref(),
                    next_left: None,
                    next_right: None,
                };
                CutLink::new(active_left.to_owned()).attach(&mut new_net, context)?;
                Ok(new_net)
            }
            DeductionRule::Tensor(tensor) => {
                let tensor_premises = proof.premises();
                if tensor_premises.len() != 2 {
                    Err(Error::BadProof)
                } else {
                    Ok(())
                }?;
                let left_premise = tensor_premises.first().unwrap().to_owned();
                let right_premise = tensor_premises.get(1).unwrap().to_owned();

                let proof_left: ProofStructure = left_premise.try_into()?;
                let proof_right: ProofStructure = right_premise.try_into()?;
                let mut new_net = proof_left;
                new_net.disjoint_union(proof_right);

                let active = tensor.get_active();
                let active_left = active.first().ok_or(Error::MissingPremise)?;
                let active_right = active.get(1).ok_or(Error::MissingPremise)?;

                let left_vert = new_net.find_conclusion(&active_left);
                let right_vert = new_net.find_conclusion(&active_right);
                let conc_vert = new_net.add_vertex(VertexLabel::C);
                let context = AttachContext {
                    prev_left: left_vert.as_ref(),
                    prev_right: right_vert.as_ref(),
                    next_left: Some(&conc_vert),
                    next_right: None,
                };
                TensorLink::new(active_left.to_owned(), active_right.to_owned())
                    .attach(&mut new_net, context)?;
                Ok(new_net)
            }
            DeductionRule::Par(par) => {
                let par_premises = proof.premises();
                if par_premises.len() != 2 {
                    Err(Error::BadProof)
                } else {
                    Ok(())
                }?;
                let left_premise = par_premises.first().unwrap().to_owned();
                let right_premise = par_premises.get(1).unwrap().to_owned();

                let proof_left: ProofStructure = left_premise.try_into()?;
                let proof_right: ProofStructure = right_premise.try_into()?;
                let mut new_net = proof_left;
                new_net.disjoint_union(proof_right);

                let active = par.get_active();
                let active_left = active.first().ok_or(Error::MissingPremise)?;
                let active_right = active.get(1).ok_or(Error::MissingPremise)?;

                let left_vert = new_net.find_conclusion(&active_left);
                let right_vert = new_net.find_conclusion(&active_right);
                let conc_vert = new_net.add_vertex(VertexLabel::C);
                let context = AttachContext {
                    prev_left: left_vert.as_ref(),
                    prev_right: right_vert.as_ref(),
                    next_left: Some(&conc_vert),
                    next_right: None,
                };
                ParLink::new(active_left.to_owned(), active_right.to_owned())
                    .attach(&mut new_net, context)?;
                Ok(new_net)
            }
            DeductionRule::Ex(_) => {
                let premises = proof.premises();
                if premises.len() != 1 {
                    Err(Error::BadProof)
                } else {
                    Ok(())
                }?;
                let premise = premises.first().unwrap().to_owned();
                premise.try_into()
            }
        }
    }
}
