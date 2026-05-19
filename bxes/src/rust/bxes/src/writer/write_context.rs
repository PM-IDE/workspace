use crate::{
  binary_rw::core::BinaryWriter,
  models::{domain::bxes_value::BxesValue, system_models::ValueAttributeDescriptor},
};
use std::{
  cell::RefCell,
  collections::{HashMap, HashSet},
  rc::Rc,
  sync::Arc,
};

pub struct BxesWriteContext<'b> {
  pub values_indices: Rc<RefCell<HashMap<Arc<BxesValue>, usize>>>,
  pub kv_indices: Rc<RefCell<HashMap<(Arc<BxesValue>, Arc<BxesValue>), usize>>>,
  pub writer: Option<&'b mut BinaryWriter<'b>>,
  pub value_attributes: Option<Vec<ValueAttributeDescriptor>>,
  pub value_attributes_set: Option<HashSet<ValueAttributeDescriptor>>,
}

impl<'b> BxesWriteContext<'b> {
  pub fn empty(value_attributes: Option<Vec<ValueAttributeDescriptor>>) -> Self {
    Self {
      values_indices: Rc::new(RefCell::new(HashMap::new())),
      kv_indices: Rc::new(RefCell::new(HashMap::new())),
      writer: None,
      value_attributes_set: Self::create_value_attributes_set(value_attributes.as_ref()),
      value_attributes,
    }
  }

  fn create_value_attributes_set(value_attributes: Option<&Vec<ValueAttributeDescriptor>>) -> Option<HashSet<ValueAttributeDescriptor>> {
    value_attributes.map(|attributes| attributes.iter().cloned().collect())
  }

  pub fn new(writer: &'b mut BinaryWriter<'b>, value_attributes: Option<Vec<ValueAttributeDescriptor>>) -> Self {
    Self {
      values_indices: Rc::new(RefCell::new(HashMap::new())),
      kv_indices: Rc::new(RefCell::new(HashMap::new())),
      writer: Some(writer),
      value_attributes_set: Self::create_value_attributes_set(value_attributes.as_ref()),
      value_attributes,
    }
  }

  pub fn with_writer<'c>(&self, writer: &'c mut BinaryWriter<'c>) -> BxesWriteContext<'c> {
    BxesWriteContext {
      values_indices: self.values_indices.clone(),
      kv_indices: self.kv_indices.clone(),
      writer: Some(writer),
      value_attributes: self.value_attributes.clone(),
      value_attributes_set: self.value_attributes_set.clone(),
    }
  }
}
