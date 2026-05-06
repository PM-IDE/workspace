use num_derive::{FromPrimitive, ToPrimitive};
use std::sync::Arc;
use variant_count::VariantCount;

use crate::models::domain::bxes_value::BxesValue;

#[derive(Debug, PartialEq, Eq)]
pub struct BxesEventLogMetadata {
  pub extensions: Option<Vec<BxesExtension>>,
  pub classifiers: Option<Vec<BxesClassifier>>,
  pub properties: Option<Vec<(Arc<BxesValue>, Arc<BxesValue>)>>,
  pub globals: Option<Vec<BxesGlobal>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BxesExtension {
  pub name: Arc<BxesValue>,
  pub prefix: Arc<BxesValue>,
  pub uri: Arc<BxesValue>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BxesClassifier {
  pub name: Arc<BxesValue>,
  pub keys: Vec<Arc<BxesValue>>,
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
  pub globals: Vec<(Arc<BxesValue>, Arc<BxesValue>)>,
}
