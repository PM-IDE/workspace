use std::collections::HashSet;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::binary_rw::core::BinaryWriter;
use crate::models::domain::bxes_value::BxesValue;
use crate::models::system_models::ValueAttributeDescriptor;

pub struct BxesWriteContext<'b> {
    pub values_indices: Rc<RefCell<HashMap<Rc<Box<BxesValue>>, usize>>>,
    pub kv_indices: Rc<RefCell<HashMap<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>), usize>>>,
    pub writer: Option<&'b mut BinaryWriter<'b>>,
    pub value_attributes: Option<Vec<ValueAttributeDescriptor>>,
    pub value_attributes_set: Option<HashSet<ValueAttributeDescriptor>>,
}

impl<'b> BxesWriteContext<'b> {
    pub fn empty() -> Self {
        Self {
            values_indices: Rc::new(RefCell::new(HashMap::new())),
            kv_indices: Rc::new(RefCell::new(HashMap::new())),
            writer: None,
            value_attributes: None,
            value_attributes_set: None,
        }
    }

    pub fn new(writer: &'b mut BinaryWriter<'b>) -> Self {
        Self {
            values_indices: Rc::new(RefCell::new(HashMap::new())),
            kv_indices: Rc::new(RefCell::new(HashMap::new())),
            writer: Some(writer),
            value_attributes: None,
            value_attributes_set: None,
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
