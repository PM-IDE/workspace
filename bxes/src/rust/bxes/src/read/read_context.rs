use std::rc::Rc;
use crate::binary_rw::core::BinaryReader;
use crate::models::domain::bxes_value::BxesValue;
use crate::models::system_models::ValueAttributeDescriptor;

pub struct ReadContext<'a> {
    pub reader: Option<&'a mut BinaryReader<'a>>,
    pub values: Option<Vec<Rc<Box<BxesValue>>>>,
    pub kv_pairs: Option<Vec<(u32, u32)>>,
    pub value_attributes: Option<Vec<ValueAttributeDescriptor>>,
}

impl<'a> ReadContext<'a> {
    pub fn new(reader: &'a mut BinaryReader<'a>) -> Self {
        Self {
            reader: Some(reader),
            values: None,
            kv_pairs: None,
            value_attributes: None
        }
    }

    pub fn new_without_reader() -> Self {
        Self {
            reader: None,
            values: None,
            kv_pairs: None,
            value_attributes: None,
        }
    }

    pub fn set_reader(&mut self, reader: &'a mut BinaryReader<'a>) {
        self.reader = Some(reader);
    }
}