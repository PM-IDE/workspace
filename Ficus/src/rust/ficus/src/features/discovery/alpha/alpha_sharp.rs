use crate::event_log::core::event_log::EventLog;
use crate::features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto, OfflineEventLogInfo};
use crate::features::discovery::alpha::alpha::find_transitions_one_length_loop;
use crate::features::discovery::alpha::providers::alpha_provider::AlphaRelationsProvider;
use crate::features::discovery::alpha::providers::alpha_sharp_provider::AlphaSharpRelationsProvider;
use crate::features::discovery::alpha::utils::maximize;
use crate::features::discovery::relations::triangle_relation::TriangleRelation;
use crate::utils::hash_utils::compare_based_on_hashes;
use std::collections::{BTreeSet, HashSet};
use std::hash::{Hash, Hasher};
use log::debug;

type AlphaSharpSet<'a> = BTreeSet<(BTreeSet<&'a String>, BTreeSet<&'a String>)>;

struct AlphaSharpTuple<'a> {
    provider: &'a AlphaSharpRelationsProvider<'a>,
    p_in: AlphaSharpSet<'a>,
    p_out: AlphaSharpSet<'a>,
}

impl<'a> AlphaSharpTuple<'a> {
    pub fn empty(provider: &'a AlphaSharpRelationsProvider<'a>) -> Self {
        Self {
            provider,
            p_in: AlphaSharpSet::new(),
            p_out: AlphaSharpSet::new(),
        }
    }

    pub fn try_create_new(
        p_in: (&'a String, &'a String),
        p_out: (&'a String, &'a String),
        provider: &'a AlphaSharpRelationsProvider<'a>,
    ) -> Option<Self> {
        let p_in = BTreeSet::from_iter(vec![(BTreeSet::from_iter(vec![p_in.0]), BTreeSet::from_iter(vec![p_in.1]))]);

        let p_out = BTreeSet::from_iter(vec![(BTreeSet::from_iter(vec![p_out.0]), BTreeSet::from_iter(vec![p_out.1]))]);

        let tuple = Self { provider, p_in, p_out };

        match tuple.valid() {
            true => Some(tuple),
            false => None,
        }
    }

    pub fn try_merge(first: &Self, second: &Self) -> Option<Self> {
        let new = Self {
            provider: first.provider,
            p_in: BTreeSet::from_iter(first.p_in.iter().chain(second.p_in.iter()).map(|c| c.clone())),
            p_out: BTreeSet::from_iter(first.p_out.iter().chain(second.p_out.iter()).map(|c| c.clone())),
        };

        match new.valid() {
            true => Some(new),
            false => None,
        }
    }

    pub fn valid(&self) -> bool {
        for in_set in &self.p_in {
            for out_set in &self.p_out {
                for a in &in_set.0 {
                    for b in &out_set.1 {
                        if !self.provider.advanced_ordering_relation(a, b) {
                            return false;
                        }
                    }
                }

                for x in &in_set.1 {
                    for y in &out_set.0 {
                        if self.provider.parallel_relation(x, y) {
                            return false;
                        }
                    }
                }
            }
        }

        let p_in = self.p_in.iter().collect::<Vec<&(BTreeSet<&'a String>, BTreeSet<&'a String>)>>();
        for i in 0..p_in.len() {
            for j in (i + 1)..p_in.len() {
                if !self.any_parallel_items(&p_in[i].0, &p_in[j].0) {
                    return false;
                }
            }
        }

        let p_out = self.p_out.iter().collect::<Vec<&(BTreeSet<&'a String>, BTreeSet<&'a String>)>>();
        for i in 0..p_out.len() {
            for j in (i + 1)..p_out.len() {
                if !self.any_parallel_items(&p_out[i].1, &p_out[j].1) {
                    return false;
                }
            }
        }

        true
    }

    fn any_parallel_items(&self, first_set: &BTreeSet<&String>, second_set: &BTreeSet<&String>) -> bool {
        let mut any_parallel = false;
        'a_set_parallel_check_loop: for first_a in first_set {
            for second_a in second_set {
                if self.provider.parallel_relation(first_a, second_a) {
                    any_parallel = true;
                    break 'a_set_parallel_check_loop;
                }
            }
        }

        any_parallel
    }
}

impl<'a> Hash for AlphaSharpTuple<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut write_tuple = |tuple: &AlphaSharpSet| {
            for set in tuple {
                for class in &set.0 {
                    state.write(class.as_bytes());
                }

                for class in &set.1 {
                    state.write(class.as_bytes());
                }
            }
        };

        write_tuple(&self.p_in);
        write_tuple(&self.p_out);
    }
}

impl<'a> PartialEq for AlphaSharpTuple<'a> {
    fn eq(&self, other: &Self) -> bool {
        compare_based_on_hashes(self, other)
    }
}

impl<'a> Eq for AlphaSharpTuple<'a> {}

impl<'a> ToString for AlphaSharpTuple<'a> {
    fn to_string(&self) -> String {
        let mut string = String::new();
        string.push('(');

        let mut push_p = |set: &AlphaSharpSet| {
            string.push('{');
            for tuple in set {
                string.push('(');
                string.push('{');
                for class in &tuple.0 {
                    string.push_str(class.as_str());
                    string.push_str(",")
                }

                if tuple.0.len() > 0 {
                    string.remove(string.len() - 1);
                }

                string.push_str("}, {");

                for class in &tuple.1 {
                    string.push_str(class.as_str());
                    string.push_str(",")
                }

                if tuple.1.len() > 0 {
                    string.remove(string.len() - 1);
                }

                string.push_str("}");
                string.push_str("),");
            }

            if set.len() > 0 {
                string.remove(string.len() - 1);
            }

            string.push_str("}, ");
        };

        push_p(&self.p_in);
        push_p(&self.p_out);

        string.remove(string.len() - 1);
        string.remove(string.len() - 1);

        string.push(')');
        string
    }
}

impl<'a> Clone for AlphaSharpTuple<'a> {
    fn clone(&self) -> Self {
        Self {
            provider: self.provider,
            p_in: BTreeSet::from_iter(self.p_in.iter().map(|t| {
                (
                    BTreeSet::from_iter(t.0.iter().map(|r| r.clone())),
                    BTreeSet::from_iter(t.0.iter().map(|r| r.clone())),
                )
            })),
            p_out: BTreeSet::from_iter(self.p_out.iter().map(|t| {
                (
                    BTreeSet::from_iter(t.0.iter().map(|r| r.clone())),
                    BTreeSet::from_iter(t.0.iter().map(|r| r.clone())),
                )
            })),
        }
    }
}

pub fn discover_petri_net_alpha_sharp(log: &impl EventLog, triangle_relation: &dyn TriangleRelation) {
    let info = OfflineEventLogInfo::create_from(EventLogInfoCreationDto::default(log));
    let one_length_loop_transitions = find_transitions_one_length_loop(log);

    let provider = AlphaSharpRelationsProvider::new(triangle_relation, &info, &one_length_loop_transitions);

    let mut advanced_pairs = HashSet::new();
    let classes = info.all_event_classes();
    for first_class in &classes {
        for second_class in &classes {
            if provider.advanced_ordering_relation(first_class, second_class)
                && !provider.redundant_advanced_ordering_relation(first_class, second_class)
            {
                advanced_pairs.insert((first_class, second_class));
            }
        }
    }

    for x in &advanced_pairs {
        debug!("({}, {})", x.0, x.1);
    }

    let mut sharp_tuples = HashSet::new();
    for pair in &advanced_pairs {
        for x_class in &classes {
            for y_class in &classes {
                let sharp_set = AlphaSharpTuple::try_create_new((pair.0, x_class), (y_class, pair.1), &provider);
                if let Some(sharp_set) = sharp_set {
                    sharp_tuples.insert(sharp_set);
                }
            }
        }
    }

    for x in &sharp_tuples {
        debug!("{}", x.to_string());
    }

    let current_set = maximize(sharp_tuples, |first, second| AlphaSharpTuple::try_merge(first, second));

    for x in &current_set {
        debug!("{}", x.to_string());
    }
}
