use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::binary_rw::core::BinaryWriter;
use crate::models::domain_models::BxesValue;

pub struct BxesWriteContext<'b> {
    pub values_indices: Rc<RefCell<HashMap<Rc<Box<BxesValue>>, usize>>>,
    pub kv_indices: Rc<RefCell<HashMap<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>), usize>>>,
    pub writer: Option<&'b mut BinaryWriter<'b>>,
}

impl<'b> BxesWriteContext<'b> {
    pub fn empty() -> Self {
        Self {
            values_indices: Rc::new(RefCell::new(HashMap::new())),
            kv_indices: Rc::new(RefCell::new(HashMap::new())),
            writer: None,
        }
    }

    pub fn new(writer: &'b mut BinaryWriter<'b>) -> Self {
        Self {
            values_indices: Rc::new(RefCell::new(HashMap::new())),
            kv_indices: Rc::new(RefCell::new(HashMap::new())),
            writer: Some(writer),
        }
    }

    pub fn with_writer<'c>(&self, writer: &'c mut BinaryWriter<'c>) -> BxesWriteContext<'c> {
        BxesWriteContext {
            values_indices: self.values_indices.clone(),
            kv_indices: self.kv_indices.clone(),
            writer: Some(writer),
        }
    }
}
