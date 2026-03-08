use crate::{
  binary_rw::core::BinaryReader,
  models::{domain::bxes_value::BxesValue, system_models::SystemMetadata},
};
use std::rc::Rc;

#[derive(Default)]
pub struct ReadMetadata {
  pub values: Option<Vec<Rc<BxesValue>>>,
  pub kv_pairs: Option<Vec<(u32, u32)>>,
  pub system_metadata: Option<SystemMetadata>,
}

pub struct ReadContext<'a, 'b> {
  pub reader: Option<&'a mut BinaryReader<'a>>,
  pub metadata: &'b mut ReadMetadata,
}

impl<'a, 'b> ReadContext<'a, 'b> {
  pub fn new(reader: &'a mut BinaryReader<'a>, metadata: &'b mut ReadMetadata) -> Self {
    Self {
      reader: Some(reader),
      metadata,
    }
  }

  pub fn new_without_reader(metadata: &'b mut ReadMetadata) -> Self {
    Self { reader: None, metadata }
  }

  pub fn set_reader(&mut self, reader: &'a mut BinaryReader<'a>) {
    self.reader = Some(reader);
  }
}
