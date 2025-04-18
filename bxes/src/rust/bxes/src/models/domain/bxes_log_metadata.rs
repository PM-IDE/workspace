use std::hash::Hash;
use std::rc::Rc;

use num_derive::{FromPrimitive, ToPrimitive};
use variant_count::VariantCount;

use crate::models::domain::bxes_value::BxesValue;

#[derive(Debug, PartialEq, Eq)]
pub struct BxesEventLogMetadata {
    pub extensions: Option<Vec<BxesExtension>>,
    pub classifiers: Option<Vec<BxesClassifier>>,
    pub properties: Option<Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>>,
    pub globals: Option<Vec<BxesGlobal>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BxesExtension {
    pub name: Rc<Box<BxesValue>>,
    pub prefix: Rc<Box<BxesValue>>,
    pub uri: Rc<Box<BxesValue>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BxesClassifier {
    pub name: Rc<Box<BxesValue>>,
    pub keys: Vec<Rc<Box<BxesValue>>>,
}

#[derive(Debug, FromPrimitive, ToPrimitive, VariantCount, PartialEq, Eq)]
pub enum BxesGlobalKind {
    Event = 0,
    Trace = 1,
    Log = 2,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BxesGlobal {
    pub entity_kind: BxesGlobalKind,
    pub globals: Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>,
}
