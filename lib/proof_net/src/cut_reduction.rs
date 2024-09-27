use super::{
    directed_multigraph::{DirectedMultiGraph, GraphEdge, GraphVertex},
    errors::Error,
    proof_structure::{Edge, ProofStructure, RuleLabel, Vertex},
};
use std::ops::Neg;

pub fn reduce(net: &mut ProofStructure) -> Result<(), Error> {
    let cuts = net.get_cuts();
    match cuts.first() {
        None => Ok(()),
        Some(next_cut) => {
            reduce_cut(net, next_cut.to_owned())?;
            reduce(net)
        }
    }
}

fn reduce_cut(net: &mut ProofStructure, v: Vertex) -> Result<(), Error> {
    let label = v.get_label();
    if label.rule != RuleLabel::Cut {
        Err(Error::WrongLabel {
            found: label.rule.clone(),
            expected: RuleLabel::Cut,
        })
    } else {
        Ok(())
    }?;
    let premises = net.get_incoming(&label)?;
    if premises.len() != 2 {
        Err(Error::BadProof)
    } else {
        Ok(())
    }?;
    let left = premises.first().unwrap();
    let right = premises.get(1).unwrap();
    match (left.from().get_label().rule, right.from().get_label().rule) {
        (RuleLabel::Ax, _) => remove_ax(net, left, right, v),
        (_, RuleLabel::Ax) => remove_ax(net, right, left, v),
        (RuleLabel::Tensor, RuleLabel::Par) => remove_par_tensor(net, v, left, right),
        (RuleLabel::Par, RuleLabel::Tensor) => remove_par_tensor(net, v, right, left),
        _ => Ok(()),
    }
}

fn remove_ax(
    net: &mut ProofStructure,
    ax_edge: &Edge,
    other_edge: &Edge,
    v: Vertex,
) -> Result<(), Error> {
    let label = v.get_label();
    let other_egde_prev = other_edge.from();
    let ax_edge_edges = net.get_outgoing(&ax_edge.from().get_label())?;
    let ax_edge_out: Vec<&Edge> = ax_edge_edges.iter().filter(|e| e.to() != v).collect();
    let ax_edge_next_edge = ax_edge_out.first().ok_or(Error::MissingConclusion)?;
    let ax_edge_next = ax_edge_next_edge.to();
    net.remove_vertex(&label)?;
    net.remove_vertex(&ax_edge.from().get_label())?;
    net.add_edge(
        &other_egde_prev.get_label(),
        &ax_edge_next.get_label(),
        ax_edge_next_edge.get_label(),
    )?;
    Ok(())
}

fn remove_par_tensor(
    net: &mut ProofStructure,
    v: Vertex,
    tensor_edge: &Edge,
    par_edge: &Edge,
) -> Result<(), Error> {
    let tensor_label = tensor_edge.from().get_label();
    let tensor_previous = net.get_incoming(&tensor_label)?;
    if tensor_previous.len() != 2 {
        Err(Error::BadProof)
    } else {
        Ok(())
    }?;
    let tensor_left = tensor_previous.first().unwrap();
    let tensor_right = tensor_previous.get(1).unwrap();

    let par_label = par_edge.from().get_label();
    let par_previous = net.get_incoming(&par_label)?;
    if par_previous.len() != 2 {
        Err(Error::BadProof)
    } else {
        Ok(())
    }?;

    let par_left = par_previous.first().unwrap();
    let par_right = par_previous.get(1).unwrap();

    let tensor_left_label = tensor_left.get_label();
    let tensor_right_label = tensor_right.get_label();
    let par_left_label = par_left.get_label();
    let par_right_label = par_right.get_label();

    let (cut1_left, cut1_right, left_label, cut2_left, cut2_right, right_label) =
        if tensor_left_label.clone().neg() == par_left_label
            && tensor_right_label.clone().neg() == par_right_label
        {
            Ok((
                tensor_left.from(),
                par_left.from(),
                tensor_right_label,
                tensor_right.from(),
                par_right.from(),
                tensor_left_label,
            ))
        } else if tensor_left_label.clone().neg() == par_right_label
            && tensor_right_label.clone().neg() == par_left_label
        {
            Ok((
                tensor_left.from(),
                par_right.from(),
                tensor_left_label,
                tensor_right.from(),
                par_left.from(),
                tensor_right_label,
            ))
        } else {
            Err(Error::BadProof)
        }?;
    net.remove_vertex(&tensor_label)?;
    net.remove_vertex(&par_label)?;
    net.remove_vertex(&v.get_label())?;
    let cut1 = net.add_vertex(net.fresh_label(RuleLabel::Cut))?;
    net.add_edge(
        &cut1_left.get_label(),
        &cut1.get_label(),
        left_label.clone(),
    )?;
    net.add_edge(
        &cut1_right.get_label(),
        &cut1.get_label(),
        right_label.clone(),
    )?;
    let cut2 = net.add_vertex(net.fresh_label(RuleLabel::Cut))?;
    net.add_edge(&cut2_left.get_label(), &cut2.get_label(), left_label.neg())?;
    net.add_edge(
        &cut2_right.get_label(),
        &cut2.get_label(),
        right_label.neg(),
    )?;

    Ok(())
}
