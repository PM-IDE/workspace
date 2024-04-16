use std::collections::HashMap;

use crate::event_log::core::event_log::EventLog;

use super::{petri_net::DefaultPetriNet, replay::replay_petri_net};

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
