use crate::models::domain::bxes_value::BxesValue;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct BxesArtifact {
    pub items: Vec<BxesArtifactItem>,
}

impl Hash for BxesArtifact {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for item in &self.items {
            item.hash(state);
        }
    }
}

impl PartialEq for BxesArtifact {
    fn eq(&self, other: &Self) -> bool {
        if self.items.len() != other.items.len() {
            return false;
        }

        for (self_item, other_item) in self.items.iter().zip(&other.items) {
            if !self_item.eq(&other_item) {
                return false;
            }
        }

        true
    }
}

#[derive(Clone, Debug)]
pub struct BxesArtifactItem {
    pub model: Rc<Box<BxesValue>>,
    pub instance: Rc<Box<BxesValue>>,
    pub transition: Rc<Box<BxesValue>>,
}

impl Hash for BxesArtifactItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.instance.hash(state);
        self.transition.hash(state);
    }
}

impl PartialEq for BxesArtifactItem {
    fn eq(&self, other: &Self) -> bool {
        self.instance == other.instance && self.transition == other.transition
    }
}
