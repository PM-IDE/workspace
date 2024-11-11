use super::{petri_net::DefaultPetriNet, replay::replay_petri_net};
use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::utils::graph::graph::DefaultGraph;
use std::collections::HashMap;
use std::str::FromStr;

pub fn annotate_with_counts(
    log: &impl EventLog,
    net: &DefaultPetriNet,
    terminate_on_unreplayable_trace: bool,
) -> Option<HashMap<u64, usize>> {
    let replay_states = replay_petri_net(log, net);
    if replay_states.is_none() {
        return None;
    }

    let mut fired_arcs = HashMap::new();
    for state in replay_states.as_ref().unwrap() {
        if terminate_on_unreplayable_trace && state.is_none() {
            return None;
        }

        if let Some(state) = state {
            for fired_transition in state.fired_transitions() {
                let transition = net.transition(fired_transition);
                for incoming_arc in transition.incoming_arcs() {
                    handle_arc(&mut fired_arcs, incoming_arc.id());
                }

                for outgoing_arc in transition.outgoing_arcs() {
                    handle_arc(&mut fired_arcs, outgoing_arc.id());
                }
            }
        }
    }

    Some(fired_arcs)
}

fn handle_arc(fired_arcs: &mut HashMap<u64, usize>, arc_id: u64) {
    *fired_arcs.entry(arc_id).or_default() += 1;
}

pub fn annotate_with_frequencies(
    log: &impl EventLog,
    net: &DefaultPetriNet,
    terminate_on_unreplayable_trace: bool,
) -> Option<HashMap<u64, f64>> {
    let count_annotation = annotate_with_counts(log, net, terminate_on_unreplayable_trace)?;
    let mut freq_annotations = HashMap::new();

    let sum: usize = count_annotation.values().into_iter().sum();
    for (arc_id, count) in count_annotation {
        freq_annotations.insert(arc_id, (count as f64) / sum as f64);
    }

    Some(freq_annotations)
}

pub fn annotate_with_trace_frequency(
    log: &impl EventLog,
    net: &DefaultPetriNet,
    terminate_on_unreplayable_trace: bool,
) -> Option<HashMap<u64, f64>> {
    let count_annotations = annotate_with_counts(log, net, terminate_on_unreplayable_trace)?;
    Some(
        count_annotations
            .into_iter()
            .map(|pair| (pair.0, pair.1 as f64 / log.traces().len() as f64))
            .collect(),
    )
}

#[derive(Copy, Clone)]
pub enum TimeAnnotationKind {
    SummedTime,
    Mean,
}

impl FromStr for TimeAnnotationKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SummedTime" => Ok(Self::SummedTime),
            "Mean" => Ok(Self::Mean),
            _ => Err(()),
        }
    }
}

pub fn annotate_with_time_performance(
    log: &impl EventLog,
    graph: &DefaultGraph,
    annotation_kind: TimeAnnotationKind,
) -> Option<HashMap<u64, f64>> {
    let mut performance_map = HashMap::new();
    for trace in log.traces() {
        let trace = trace.borrow();
        let events = trace.events();
        for i in 0..(events.len() - 1) {
            let first = events.get(i).expect("Index in bounds");
            let first = first.borrow();

            let second = events.get(i + 1).expect("Index in bounds");
            let second = second.borrow();

            if first.timestamp() > second.timestamp() {
                println!("Encountered broken trace, first.timestamp() > second.timestamp(), {}", i);
                continue;
            }

            let time_diff = second.timestamp().to_owned() - first.timestamp().to_owned();
            let time_diff = time_diff.num_nanoseconds().expect("Must be convertible to nanos") as f64;

            let key = (first.name().to_owned(), second.name().to_owned());
            if let Some((existing_time_diff, count)) = performance_map.get(&key) {
                *performance_map.get_mut(&key).expect("Must exist") = (*existing_time_diff + time_diff, *count + 1);
            } else {
                performance_map.insert(key, (time_diff, 1i64));
            }
        }
    }

    let mut time_annotations = HashMap::new();
    for edge in graph.all_edges() {
        let first_node = graph.node(&edge.first_node_id).expect("Must contain first node");
        let second_node = graph.node(&edge.second_node_id).expect("Must contain second node");

        let key = (
            first_node.data.as_ref().unwrap().to_owned(),
            second_node.data.as_ref().unwrap().to_owned(),
        );

        if let Some(time_annotation) = performance_map.get(&key) {
            let time_annotation = match annotation_kind {
                TimeAnnotationKind::SummedTime => time_annotation.0,
                TimeAnnotationKind::Mean => time_annotation.0 / time_annotation.1 as f64,
            };

            time_annotations.insert(*edge.id(), time_annotation);
        }
    }

    Some(time_annotations)
}
