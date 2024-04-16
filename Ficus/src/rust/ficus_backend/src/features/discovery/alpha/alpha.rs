use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto};
use crate::features::discovery::alpha::alpha_set::AlphaSet;
use crate::features::discovery::alpha::providers::alpha_plus_provider::AlphaPlusRelationsProvider;
use crate::features::discovery::alpha::providers::alpha_provider::AlphaRelationsProvider;
use crate::features::discovery::alpha::utils::maximize;
use crate::features::discovery::petri_net::marking::{Marking, SingleMarking};
use crate::features::discovery::petri_net::petri_net::{DefaultPetriNet, PetriNet};
use crate::features::discovery::petri_net::place::Place;
use crate::features::discovery::petri_net::transition::Transition;
use crate::utils::user_data::keys::DefaultKey;
use crate::utils::user_data::user_data::UserData;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::string::ToString;

pub static ALPHA_SET: Lazy<DefaultKey<AlphaSet>> = Lazy::new(|| DefaultKey::new("alpha_set".to_string()));

pub fn discover_petri_net_alpha(provider: &impl AlphaRelationsProvider) -> DefaultPetriNet {
    do_discover_petri_net_alpha(provider)
}

pub fn discover_petri_net_alpha_plus(
    log: &impl EventLog,
    provider: &impl AlphaPlusRelationsProvider,
    alpha_plus_plus: bool,
) -> DefaultPetriNet {
    let mut petri_net = do_discover_petri_net_alpha(provider);
    add_one_length_loops(log, provider.one_length_loop_transitions(), &mut petri_net);

    if alpha_plus_plus {
        add_alpha_plus_plus_transitions(log, provider.one_length_loop_transitions(), &mut petri_net);
    }

    petri_net
}

fn add_one_length_loops(log: &impl EventLog, one_length_loop_transitions: &HashSet<String>, petri_net: &mut DefaultPetriNet) {
    let event_log_info = EventLogInfo::create_from(EventLogInfoCreationDto::default(log));

    for transition_name in one_length_loop_transitions {
        let mut alpha_set = AlphaSet::empty();
        if let Some(followed_events) = event_log_info.dfg_info().get_followed_events(transition_name) {
            for event in followed_events.keys() {
                if event != transition_name {
                    alpha_set.insert_right_class(event.to_owned());
                }
            }
        }

        if let Some(precedes_events) = event_log_info.dfg_info().get_precedes_events(transition_name) {
            for event in precedes_events.keys() {
                if event != transition_name {
                    alpha_set.insert_left_class(event.to_owned());
                }
            }
        }

        let id = petri_net.add_transition(Transition::empty(
            transition_name.to_owned(),
            false,
            Some(transition_name.to_owned()),
        ));

        let place_id = match petri_net.find_place_id_by_name(alpha_set.to_string().as_str()) {
            Some(found_place_id) => found_place_id,
            None => petri_net.add_place(Place::with_name(alpha_set.to_string())),
        };

        petri_net.connect_transition_to_place(&id, &place_id, None);
        petri_net.connect_place_to_transition(&place_id, &id, None);
    }
}

fn add_alpha_plus_plus_transitions(log: &impl EventLog, one_length_loop_transitions: &HashSet<String>, petri_net: &mut DefaultPetriNet) {
    let key = Lazy::get(&ALPHA_SET).unwrap();
    let mut transitions_connections = HashSet::new();
    let mut places_connections = HashSet::new();

    for transition in one_length_loop_transitions {
        if let Some(transition) = petri_net.find_transition_by_name(transition) {
            for place in petri_net.all_places() {
                if let Some(alpha_set) = place.user_data().concrete(key) {
                    for outgoing_arc in transition.outgoing_arcs() {
                        if outgoing_arc.place_id() != place.id() {
                            let outgoing_place = petri_net.place(&outgoing_arc.place_id());
                            if let Some(outgoing_alpha_set) = outgoing_place.user_data().concrete(key) {
                                if alpha_set.is_full_subset(outgoing_alpha_set) {
                                    transitions_connections.insert((transition.id(), outgoing_place.id()));
                                }
                            }
                        }
                    }

                    for incoming_arc in transition.incoming_arcs() {
                        if incoming_arc.place_id() != place.id() {
                            let incoming_place = petri_net.place(&incoming_arc.place_id());
                            if let Some(incoming_alpha_set) = incoming_place.user_data().concrete(key) {
                                if alpha_set.is_full_subset(incoming_alpha_set) {
                                    places_connections.insert((incoming_place.id(), transition.id()));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    for connection in transitions_connections {
        petri_net.connect_transition_to_place(&connection.0, &connection.1, None);
    }

    for connection in places_connections {
        petri_net.connect_place_to_transition(&connection.0, &connection.1, None);
    }
}

pub fn find_transitions_one_length_loop(log: &impl EventLog) -> HashSet<String> {
    let mut one_loop_transitions = HashSet::new();
    for trace in log.traces() {
        let trace = trace.borrow();
        let events = trace.events();
        for i in 0..(events.len() - 1) {
            if events[i].borrow().name() == events[i + 1].borrow().name() {
                one_loop_transitions.insert(events[i].borrow().name().to_owned());
            }
        }
    }

    one_loop_transitions
}

fn do_discover_petri_net_alpha(provider: &impl AlphaRelationsProvider) -> DefaultPetriNet {
    let mut current_sets = create_initial_sets(provider);
    current_sets = maximize_sets(current_sets, provider);

    create_petri_net(provider.log_info(), filter_out_non_maximal_sets(&current_sets))
}

fn create_initial_sets(provider: &impl AlphaRelationsProvider) -> HashSet<AlphaSet> {
    let info = provider.log_info();

    info.all_event_classes()
        .iter()
        .filter(|class| info.dfg_info().get_followed_events(class).is_some() && provider.unrelated_relation(class, class))
        .flat_map(|class| {
            let mut sets = vec![];
            for candidate in info.all_event_classes() {
                if provider.causal_relation(class, candidate) && provider.unrelated_relation(candidate, candidate) {
                    sets.push(AlphaSet::new((*class).to_owned(), candidate.to_owned()));
                }
            }

            sets
        })
        .filter(|set| set.left_classes().len() > 0 && set.right_classes().len() > 0)
        .collect()
}

fn maximize_sets(current_sets: HashSet<AlphaSet>, provider: &impl AlphaRelationsProvider) -> HashSet<AlphaSet> {
    maximize(current_sets, |first, second| {
        let should_extend = (first.is_left_subset(second) || first.is_right_subset(second)) && first.can_extend(second, provider);

        match should_extend {
            true => Some(first.extend(&second)),
            false => None,
        }
    })
}

fn filter_out_non_maximal_sets(current_sets: &HashSet<AlphaSet>) -> Vec<&AlphaSet> {
    current_sets
        .iter()
        .filter(|pair| {
            !current_sets
                .iter()
                .any(|candidate| *pair != candidate && pair.is_full_subset(candidate))
        })
        .collect()
}

fn create_petri_net(info: &EventLogInfo, alpha_sets: Vec<&AlphaSet>) -> DefaultPetriNet {
    let mut petri_net = PetriNet::empty();
    let mut event_classes_to_transition_ids = HashMap::new();
    for class in info.all_event_classes() {
        let id = petri_net.add_transition(Transition::empty(class.to_owned(), false, Some(class.to_owned())));
        event_classes_to_transition_ids.insert(class, id);
    }

    for alpha_set in alpha_sets {
        let mut place = Place::with_name(alpha_set.to_string());
        place.user_data_mut().put_concrete(&ALPHA_SET, alpha_set.clone());
        let place_id = petri_net.add_place(place);

        for class in alpha_set.left_classes() {
            petri_net.connect_transition_to_place(&event_classes_to_transition_ids[&class], &place_id, None);
        }

        for class in alpha_set.right_classes() {
            petri_net.connect_place_to_transition(&place_id, &event_classes_to_transition_ids[&class], None);
        }
    }

    let start_place_id = petri_net.add_place(Place::with_name("StartPlace".to_string()));
    for start_activity in info.start_event_classes() {
        petri_net.connect_place_to_transition(&start_place_id, &event_classes_to_transition_ids[start_activity], None);
    }

    let end_place_id = petri_net.add_place(Place::with_name("EndPlace".to_string()));
    for end_activity in info.end_event_classes() {
        petri_net.connect_transition_to_place(&event_classes_to_transition_ids[end_activity], &end_place_id, None);
    }

    petri_net.set_initial_marking(Marking::new(vec![SingleMarking::new(start_place_id, 1)]));
    petri_net.set_final_marking(Marking::new(vec![SingleMarking::new(end_place_id, 1)]));

    petri_net
}
