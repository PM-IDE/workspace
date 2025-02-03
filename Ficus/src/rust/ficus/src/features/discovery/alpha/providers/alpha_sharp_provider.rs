use crate::features::analysis::log_info::event_log_info::{EventLogInfo, OfflineEventLogInfo};
use crate::features::discovery::alpha::providers::alpha_plus_provider::{AlphaPlusRelationsProvider, AlphaPlusRelationsProviderImpl};
use crate::features::discovery::alpha::providers::alpha_provider::AlphaRelationsProvider;
use crate::features::discovery::relations::triangle_relation::TriangleRelation;
use std::collections::HashSet;

pub struct AlphaSharpRelationsProvider<'a> {
    alpha_plus_provider: AlphaPlusRelationsProviderImpl<'a>,
    info: &'a dyn EventLogInfo,
}

impl<'a> AlphaRelationsProvider for AlphaSharpRelationsProvider<'a> {
    fn causal_relation(&self, first: &str, second: &str) -> bool {
        self.alpha_plus_provider.causal_relation(first, second)
    }

    fn parallel_relation(&self, first: &str, second: &str) -> bool {
        self.alpha_plus_provider.parallel_relation(first, second)
    }

    fn direct_relation(&self, first: &str, second: &str) -> bool {
        self.alpha_plus_provider.direct_relation(first, second)
    }

    fn unrelated_relation(&self, first: &str, second: &str) -> bool {
        self.alpha_plus_provider.unrelated_relation(first, second)
    }

    fn log_info(&self) -> &dyn EventLogInfo {
        self.alpha_plus_provider.log_info()
    }
}

impl<'a> AlphaSharpRelationsProvider<'a> {
    pub fn triangle_relation(&self, first: &str, second: &str) -> bool {
        self.alpha_plus_provider.triangle_relation(first, second)
    }

    pub fn romb_relation(&self, first: &str, second: &str) -> bool {
        self.alpha_plus_provider.romb_relation(first, second)
    }

    pub fn advanced_ordering_relation(&self, first: &str, second: &str) -> bool {
        self.alpha_plus_provider.causal_relation(first, second) && self.check_advanced_ordering_relation_second_part(first, second)
    }

    fn check_advanced_ordering_relation_second_part(&self, first: &str, second: &str) -> bool {
        let classes = self.info.all_event_classes();
        for x_class in &classes {
            for y_class in &classes {
                let first_causal_x = self.causal_relation(first, x_class);
                let y_causal_second = self.causal_relation(y_class, second);
                let x_following_y = self.direct_relation(y_class, x_class);
                let x_parallel_second = self.parallel_relation(x_class, second);
                let first_parallel_y = self.parallel_relation(first, y_class);

                if first_causal_x && y_causal_second && !x_following_y && !x_parallel_second && !first_parallel_y {
                    return true;
                }
            }
        }

        false
    }

    pub fn real_causal_dependency(&self, first: &str, second: &str) -> bool {
        self.causal_relation(first, second) && !self.advanced_ordering_relation(first, second)
    }

    pub fn redundant_advanced_ordering_relation(&self, first: &str, second: &str) -> bool {
        let classes = self.info.all_event_classes();
        for c_class in &classes {
            for d_class in &classes {
                if c_class.as_str() != first && d_class.as_str() != second {
                    let c_causal_d = self.causal_relation(c_class, d_class);
                    let first_advanced_d = self.advanced_ordering_relation(first, d_class);
                    let c_advanced_second = self.advanced_ordering_relation(c_class, second);

                    if c_causal_d && first_advanced_d && c_advanced_second {
                        return true;
                    }
                }
            }
        }

        false
    }
}

impl<'a> AlphaSharpRelationsProvider<'a> {
    pub fn new(
        triangle_relation: &'a dyn TriangleRelation,
        info: &'a OfflineEventLogInfo,
        one_length_loop_transitions: &'a HashSet<String>,
    ) -> Self {
        Self {
            alpha_plus_provider: AlphaPlusRelationsProviderImpl::new(info, triangle_relation, one_length_loop_transitions),
            info,
        }
    }
}
