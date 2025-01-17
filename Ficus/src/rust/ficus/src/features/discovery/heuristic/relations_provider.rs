use crate::event_log::core::event_log::EventLog;
use crate::features::analysis::event_log_info::{EventLogInfo, OfflineEventLogInfo};
use crate::features::discovery::alpha::providers::alpha_plus_provider::calculate_triangle_relations;
use crate::features::discovery::alpha::providers::alpha_provider::{AlphaRelationsProvider, DefaultAlphaRelationsProvider};
use std::collections::HashMap;

type DependencyRelations = HashMap<String, HashMap<String, f64>>;

pub(crate) struct HeuristicMinerRelationsProvider<'a> {
    dependency_threshold: f64,
    positive_observations_threshold: usize,
    relative_to_best_threshold: f64,
    and_threshold: f64,
    loop_length_two_threshold: f64,
    triangle_relations: HashMap<(String, String), usize>,
    provider: DefaultAlphaRelationsProvider<'a>,
    dependency_relations: DependencyRelations,
}

#[derive(PartialEq)]
pub enum AndOrXorRelation {
    And,
    Xor,
}

impl<'a> HeuristicMinerRelationsProvider<'a> {
    pub fn new(
        log: &impl EventLog,
        provider: DefaultAlphaRelationsProvider<'a>,
        dependency_threshold: f64,
        positive_observations_threshold: usize,
        relative_to_best_threshold: f64,
        and_threshold: f64,
        loop_length_two_threshold: f64,
    ) -> Self {
        let mut provider = Self {
            triangle_relations: calculate_triangle_relations(log),
            dependency_threshold,
            positive_observations_threshold,
            relative_to_best_threshold,
            loop_length_two_threshold,
            provider,
            dependency_relations: DependencyRelations::new(),
            and_threshold,
        };

        provider.initialize_dependency_relations();
        provider
    }

    fn initialize_dependency_relations(&mut self) {
        let mut relations = HashMap::<String, Vec<(String, f64)>>::new();
        for first_class in self.provider.log_info().all_event_classes() {
            for second_class in self.provider.log_info().all_event_classes() {
                let second_follows_first = self.get_directly_follows_count(first_class, second_class);
                if second_follows_first < self.positive_observations_threshold {
                    continue;
                }

                let measure = self.calculate_dependency_measure(first_class, second_class);
                if measure <= self.dependency_threshold {
                    continue;
                }

                if let Some(values) = relations.get_mut(first_class) {
                    values.push((second_class.to_owned(), measure));
                } else {
                    relations.insert(first_class.to_owned(), vec![(second_class.to_owned(), measure)]);
                }
            }
        }

        for key in self.provider.log_info().all_event_classes() {
            if let Some(values) = relations.get_mut(key.as_str()) {
                let best_value = values.iter().max_by(|first, second| first.1.total_cmp(&second.1)).unwrap().1;

                let min_value = best_value * (1.0 - self.relative_to_best_threshold);
                for i in (0..values.len()).rev() {
                    if values[i].1 < min_value {
                        values.remove(i);
                    }
                }
            }
        }

        for (key, values) in relations {
            let mut map = HashMap::new();
            for (second_key, value) in values {
                map.insert(second_key, value);
            }

            self.dependency_relations.insert(key, map);
        }
    }

    pub fn dependency_relation(&self, first: &str, second: &str) -> bool {
        if let Some(values) = self.dependency_relations.get(first) {
            values.contains_key(second)
        } else {
            false
        }
    }

    pub fn and_or_xor_relation(&self, a: &String, b: &String, c: &String) -> AndOrXorRelation {
        let b_c = self.get_directly_follows_count(b, c) as f64;
        let c_b = self.get_directly_follows_count(c, b) as f64;
        let a_b = self.get_directly_follows_count(a, b) as f64;
        let a_c = self.get_directly_follows_count(a, c) as f64;

        let and_xor_measure = (b_c + c_b) / (a_b + a_c + 1.0);

        if and_xor_measure > self.and_threshold {
            AndOrXorRelation::And
        } else {
            AndOrXorRelation::Xor
        }
    }

    pub fn loop_length_two_relation(&self, first: &str, second: &str) -> bool {
        let a_b = self.triangle_occurrences_count(first, second) as f64;
        let b_a = self.triangle_occurrences_count(second, first) as f64;

        (a_b + b_a) / (a_b + b_a + 1.0) > self.loop_length_two_threshold
    }

    fn calculate_dependency_measure(&self, first: &String, second: &String) -> f64 {
        let b_follows_a = self.get_directly_follows_count(first, second) as f64;
        let a_follows_b = self.get_directly_follows_count(second, first) as f64;

        if first != second {
            (b_follows_a - a_follows_b) / (b_follows_a + a_follows_b + 1.0)
        } else {
            a_follows_b / (a_follows_b + 1.0)
        }
    }

    fn get_directly_follows_count(&self, first: &String, second: &String) -> usize {
        self.provider.log_info().dfg_info().get_directly_follows_count(first, second)
    }

    fn triangle_occurrences_count(&self, first: &str, second: &str) -> usize {
        if let Some(measure) = self.triangle_relations.get(&(first.to_owned(), second.to_owned())) {
            *measure
        } else {
            0
        }
    }

    pub fn log_info(&self) -> &dyn EventLogInfo {
        self.provider.log_info()
    }
}
