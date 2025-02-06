use crate::features::analysis::log_info::event_log_info::EventLogInfo;
use crate::features::discovery::alpha::providers::alpha_provider::DefaultAlphaRelationsProvider;
use crate::features::discovery::alpha::utils::maximize;
use crate::features::discovery::heuristic::relations_provider::{AndOrXorRelation, HeuristicMinerRelationsProvider};
use crate::features::discovery::petri_net::petri_net::DefaultPetriNet;
use crate::features::discovery::petri_net::place::Place;
use crate::features::discovery::petri_net::transition::Transition;
use crate::features::discovery::relations::triangle_relation::TriangleRelation;
use crate::utils::sets::one_set::OneSet;
use std::collections::{HashMap, HashSet};

pub fn discover_petri_net_heuristic(
    info: &dyn EventLogInfo,
    triangle_relation: &dyn TriangleRelation,
    dependency_threshold: f64,
    positive_observations_threshold: usize,
    relative_to_best_threshold: f64,
    and_threshold: f64,
    loop_length_two_threshold: f64,
) -> DefaultPetriNet {
    let provider = DefaultAlphaRelationsProvider::new(info);
    let provider = HeuristicMinerRelationsProvider::new(
        triangle_relation,
        provider,
        dependency_threshold,
        positive_observations_threshold,
        relative_to_best_threshold,
        and_threshold,
        loop_length_two_threshold,
    );

    let mut petri_net = DefaultPetriNet::empty();

    construct_heuristic_petri_net(&provider, &mut petri_net);
    add_length_two_loops(info, &provider, &mut petri_net);

    petri_net
}

fn construct_heuristic_petri_net(provider: &HeuristicMinerRelationsProvider, petri_net: &mut DefaultPetriNet) {
    let mut classes_to_ids = HashMap::new();
    for class in provider.log_info().all_event_classes() {
        let id = petri_net.add_transition(Transition::empty(class.to_owned(), false, Some(class.to_owned())));
        classes_to_ids.insert(class.to_owned(), id);
    }

    for first_class in provider.log_info().all_event_classes() {
        let mut followers = Vec::new();
        for second_class in provider.log_info().all_event_classes() {
            if provider.dependency_relation(first_class, second_class) {
                followers.push(second_class);
            }
        }

        if followers.len() == 0 {
            continue;
        }

        let mut and_relations = HashSet::new();
        for i in 0..followers.len() {
            for j in (i + 1)..followers.len() {
                let first = *followers.get(i).unwrap();
                let second = *followers.get(j).unwrap();

                if first != second && provider.and_or_xor_relation(first_class, first, second) == AndOrXorRelation::And {
                    and_relations.insert(OneSet::new_two_elements(first, second));
                }
            }
        }

        let parallel_groups = maximize(and_relations, |first, second| {
            let candidate = first.merge(second);
            for first_el in candidate.set() {
                for second_el in candidate.set() {
                    if first_el != second_el && provider.and_or_xor_relation(first_class, first_el, second_el) != AndOrXorRelation::And {
                        return None;
                    }
                }
            }

            Some(candidate)
        });

        let mut used = HashSet::new();
        let post_place_id = petri_net.add_place(Place::with_name(format!("post_{first_class}")));
        petri_net.connect_transition_to_place(classes_to_ids.get(first_class).unwrap(), &post_place_id, None);

        for group in &parallel_groups {
            let name = format!("silent_start_{first_class}");
            let id = petri_net.add_transition(Transition::empty(name.to_owned(), true, Some(name.to_owned())));
            petri_net.connect_place_to_transition(&post_place_id, &id, None);

            for el in group.set().iter() {
                let place_id = petri_net.add_place(Place::with_name(format!("pre_{el}")));
                petri_net.connect_transition_to_place(&id, &place_id, None);
                petri_net.connect_place_to_transition(&place_id, classes_to_ids.get(*el).unwrap(), None);

                used.insert(*el);
            }
        }

        for follower in &followers {
            if !used.contains(follower) {
                petri_net.connect_place_to_transition(&post_place_id, classes_to_ids.get(*follower).unwrap(), None);
            }
        }
    }
}

fn add_length_two_loops(info: &dyn EventLogInfo, provider: &HeuristicMinerRelationsProvider, petri_net: &mut DefaultPetriNet) {
    let mut places_to_transitions = vec![];
    let mut transitions_to_places = vec![];
    for first_class in info.all_event_classes() {
        for second_class in info.all_event_classes() {
            if first_class != second_class && provider.loop_length_two_relation(first_class, second_class) {
                let first_transition = petri_net.find_transition_by_name(first_class).unwrap();
                let second_transition = petri_net.find_transition_by_name(second_class).unwrap();

                for output_arc in second_transition.outgoing_arcs() {
                    places_to_transitions.push((output_arc.place_id(), first_transition.id()));
                }

                for incoming_arc in second_transition.incoming_arcs() {
                    transitions_to_places.push((first_transition.id(), incoming_arc.place_id()));
                }
            }
        }
    }

    for (place_id, transition_id) in places_to_transitions {
        petri_net.connect_place_to_transition(&place_id, &transition_id, None);
    }

    for (transition_id, place_id) in transitions_to_places {
        petri_net.connect_transition_to_place(&transition_id, &place_id, None);
    }
}
