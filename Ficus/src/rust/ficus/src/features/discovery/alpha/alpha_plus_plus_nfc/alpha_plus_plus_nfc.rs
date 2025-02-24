use crate::event_log::core::event_log::EventLog;
use crate::features::analysis::log_info::event_log_info::{EventLogInfo, OfflineEventLogInfo};
use crate::features::analysis::log_info::log_info_creation_dto::EventLogInfoCreationDto;
use crate::features::discovery::alpha::alpha::{
  discover_petri_net_alpha, discover_petri_net_alpha_plus, find_transitions_one_length_loop, ALPHA_SET,
};
use crate::features::discovery::alpha::alpha_plus_plus_nfc::alpha_plus_plus_nfc_triple::AlphaPlusPlusNfcTriple;
use crate::features::discovery::alpha::alpha_plus_plus_nfc::extended_alpha_set::ExtendedAlphaSet;
use crate::features::discovery::alpha::alpha_plus_plus_nfc::w3_pair::W3Pair;
use crate::features::discovery::alpha::providers::alpha_plus_nfc_provider::AlphaPlusNfcRelationsProvider;
use crate::features::discovery::alpha::utils::maximize;
use crate::features::discovery::petri_net::petri_net::DefaultPetriNet;
use crate::features::discovery::petri_net::place::Place;
use crate::features::discovery::petri_net::transition::Transition;
use crate::features::discovery::relations::triangle_relation::{OfflineTriangleRelation, TriangleRelation};
use crate::utils::sets::two_sets::TwoSets;
use crate::utils::user_data::user_data::UserData;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn discover_petri_net_alpha_plus_plus_nfc<TLog: EventLog>(log: &TLog) -> DefaultPetriNet {
  let one_length_loop_transitions = find_transitions_one_length_loop(log);
  let info = OfflineEventLogInfo::create_from(EventLogInfoCreationDto::default(log));
  let triangle_relation = OfflineTriangleRelation::new(log);

  let provider = AlphaPlusNfcRelationsProvider::new(&info, log, &triangle_relation, &one_length_loop_transitions);

  let mut x_w = HashSet::new();

  for a_class in info.all_event_classes() {
    for b_class in info.all_event_classes() {
      for c_class in &one_length_loop_transitions {
        if let Some(triple) = AlphaPlusPlusNfcTriple::try_new(a_class, b_class, c_class, &provider) {
          x_w.insert(triple);
        }
      }
    }
  }

  let l_w = maximize(x_w, |first, second| AlphaPlusPlusNfcTriple::try_merge(first, second, &provider));

  let petri_net = discover_petri_net_alpha_plus(&provider, &info, false);

  let info = OfflineEventLogInfo::create_from(EventLogInfoCreationDto::default_ignore(log, &one_length_loop_transitions));
  let mut provider = AlphaPlusNfcRelationsProvider::new(&info, log, &triangle_relation, &one_length_loop_transitions);

  let mut w1_relations = HashSet::new();
  for a_class in info.all_event_classes() {
    for b_class in info.all_event_classes() {
      if provider.w1_relation(a_class, b_class, &petri_net) {
        w1_relations.insert((a_class, b_class));
      }
    }
  }

  for pair in &w1_relations {
    provider.add_additional_causal_relation(pair.0, pair.1);
  }

  let petri_net = discover_petri_net_alpha_plus(&provider, &info, false);

  let mut w2_relations = HashSet::new();
  for a_class in info.all_event_classes() {
    for b_class in info.all_event_classes() {
      if provider.w2_relation(a_class, b_class, &petri_net) {
        w2_relations.insert((a_class, b_class));
      }
    }
  }

  eliminate_by_reduction_rule_1(&mut w2_relations, &mut provider, &petri_net, &info);

  let mut x_w = HashSet::new();
  let alpha_net = discover_petri_net_alpha(&provider);

  for place in alpha_net.all_places() {
    if let Some(alpha_set) = place.user_data().concrete(&ALPHA_SET) {
      for first_class in info.all_event_classes() {
        for second_class in info.all_event_classes() {
          let set = ExtendedAlphaSet::try_new(
            alpha_set.clone(),
            first_class,
            second_class,
            &mut provider,
            &w1_relations,
            &w2_relations,
          );
          if let Some(extended_alpha_set) = set {
            x_w.insert(extended_alpha_set);
          }
        }
      }

      for class in info.all_event_classes() {
        let set = ExtendedAlphaSet::try_new_only_left(alpha_set.clone(), class, &mut provider, &w1_relations, &w2_relations);
        if let Some(set) = set {
          x_w.insert(set);
        }

        let set = ExtendedAlphaSet::try_new_only_right(alpha_set.clone(), class, &mut provider, &w1_relations, &w2_relations);
        if let Some(set) = set {
          x_w.insert(set);
        }
      }
    }
  }

  for place in alpha_net.all_places() {
    if let Some(alpha_set) = place.user_data().concrete(&ALPHA_SET) {
      x_w.insert(ExtendedAlphaSet::new_without_extensions(alpha_set.clone()));
    }
  }

  let y_w = maximize(x_w, |first, second| {
    if !first.alpha_set().eq(second.alpha_set()) {
      return None;
    }

    let new = first.merge(second);
    if new.valid(&mut provider, &w1_relations, &w2_relations) {
      Some(new)
    } else {
      None
    }
  });

  for w2_relation in &w2_relations {
    provider.add_additional_causal_relation(w2_relation.0, w2_relation.1);
  }

  let petri_net = discover_petri_net_alpha_plus(&provider, &info, false);

  let mut w3_relations = HashSet::new();
  for a_class in info.all_event_classes() {
    for b_class in info.all_event_classes() {
      if provider.w3_relation(a_class, b_class, &petri_net) {
        w3_relations.insert((a_class, b_class));
      }
    }
  }

  let w3_closure = construct_w3_transitive_closure_cache(&w3_relations);
  eliminate_w3_relations_by_rule_2(&mut w3_relations, &w3_closure);

  let mut x_w = HashSet::new();
  for first_class in info.all_event_classes() {
    for second_class in info.all_event_classes() {
      if let Some(pair) = W3Pair::try_new(first_class, second_class, &w3_relations, &provider) {
        x_w.insert(pair);
      }
    }
  }

  let z_w = maximize(x_w, |first, second| {
    let new_pair = first.merge(second);
    if new_pair.valid(&w3_relations, &provider) {
      Some(new_pair)
    } else {
      None
    }
  });

  let mut p_w = HashSet::new();
  let check_should_add_to_pw = |two_sets: &TwoSets<&String>| {
    for l_w_item in &l_w {
      if l_w_item.a_classes().eq(&two_sets.first_set()) && l_w_item.b_classes().eq(&two_sets.second_set()) {
        return false;
      }
    }

    true
  };

  for item in &z_w {
    if check_should_add_to_pw(&item.two_sets()) {
      p_w.insert(item.two_sets());
    }
  }

  for item in &y_w {
    let two_sets = item.two_sets();
    if check_should_add_to_pw(&two_sets) {
      p_w.insert(two_sets);
    }
  }

  for l_w_item in &l_w {
    p_w.insert(l_w_item.two_sets());
  }

  let mut resulting_net = DefaultPetriNet::empty();
  let mut transitions_to_ids = HashMap::new();
  for transition in info
    .all_event_classes()
    .iter()
    .map(|c| *c)
    .chain(one_length_loop_transitions.iter())
  {
    let id = resulting_net.add_transition(Transition::empty((*transition).to_owned(), false, Some((*transition).to_owned())));
    transitions_to_ids.insert(transition, id);
  }

  for place in &p_w {
    let place_id = resulting_net.add_place(Place::with_name(place.to_string()));
    for transition in place.first_set() {
      resulting_net.connect_transition_to_place(transitions_to_ids.get(transition).unwrap(), &place_id, None);
    }

    for transition in place.second_set() {
      resulting_net.connect_place_to_transition(&place_id, transitions_to_ids.get(transition).unwrap(), None);
    }
  }

  resulting_net
}

fn eliminate_by_reduction_rule_1<TLog: EventLog>(
  w2_relations: &mut HashSet<(&String, &String)>,
  provider: &mut AlphaPlusNfcRelationsProvider<TLog>,
  petri_net: &DefaultPetriNet,
  info: &OfflineEventLogInfo,
) {
  let mut to_remove = Vec::new();
  for w2_relation in w2_relations.iter() {
    let a = w2_relation.0;
    let c = w2_relation.1;
    for b in info.all_event_classes() {
      if (provider.w2_relation(a, b, petri_net) && provider.concave_arrow_relation(b, c))
        || (provider.w2_relation(b, c, petri_net) && provider.concave_arrow_relation(a, b))
      {
        to_remove.push(w2_relation.clone());
      }
    }
  }

  for item in &to_remove {
    w2_relations.remove(item);
  }
}

fn construct_w3_transitive_closure_cache<'a>(w3_relations: &'a HashSet<(&'a String, &'a String)>) -> HashMap<String, HashSet<String>> {
  let mut graph: HashMap<&String, HashSet<&String>> = HashMap::new();
  let mut all_classes = HashSet::new();
  for relation in w3_relations {
    if let Some(children) = graph.get_mut(relation.0) {
      children.insert(relation.1);
    } else {
      graph.insert(relation.0, HashSet::from_iter(vec![relation.1]));
    }

    all_classes.insert(relation.0);
    all_classes.insert(relation.1);
  }

  let mut closure: HashMap<String, HashSet<String>> = HashMap::new();

  for first_class in &all_classes {
    for second_class in &all_classes {
      if let Some(children) = graph.get(first_class) {
        if children.contains(second_class) {
          continue;
        }
      }

      let mut is_in_closure = false;
      let mut q = VecDeque::new();
      q.push_back(first_class);

      'q_loop: while !q.is_empty() {
        let current_class = q.pop_front().unwrap();
        if let Some(children) = graph.get(current_class) {
          if children.contains(second_class) {
            is_in_closure = true;
            break 'q_loop;
          } else {
            for child in children {
              q.push_back(child);
            }
          }
        }
      }

      if is_in_closure {
        if let Some(children) = closure.get_mut(*first_class) {
          children.insert((**second_class).clone());
        } else {
          closure.insert((**first_class).clone(), HashSet::from_iter(vec![(**second_class).clone()]));
        }
      }
    }
  }

  closure
}

fn eliminate_w3_relations_by_rule_2(w3_relations: &mut HashSet<(&String, &String)>, closure_cache: &HashMap<String, HashSet<String>>) {
  let mut to_remove = HashSet::new();
  for relation in w3_relations.iter() {
    if let Some(children) = closure_cache.get(relation.0) {
      if children.contains(relation.1) {
        to_remove.insert(relation.clone());
      }
    }
  }

  for item in &to_remove {
    w3_relations.remove(item);
  }
}
