use crate::binary_rw::core::BinaryReader;
use crate::models::domain::bxes_value::BxesValue;
use crate::models::system_models::SystemMetadata;
use std::rc::Rc;

pub struct ReadContext<'a> {
    pub reader: Option<&'a mut BinaryReader<'a>>,
    pub values: Option<Vec<Rc<Box<BxesValue>>>>,
    pub kv_pairs: Option<Vec<(u32, u32)>>,
    pub system_metadata: Option<SystemMetadata>,
}

impl<'a> ReadContext<'a> {
    pub fn new(reader: &'a mut BinaryReader<'a>) -> Self {
        Self {
            reader: Some(reader),
            values: None,
            kv_pairs: None,
            system_metadata: None,
        }
    }

    pub fn new_without_reader() -> Self {
        Self {
            reader: None,
            values: None,
            kv_pairs: None,
            system_metadata: None,
        }
    }

    pub fn set_reader(&mut self, reader: &'a mut BinaryReader<'a>) {
        self.reader = Some(reader);
    }

    pub fn clone_with_new_reader(self, new_reader: &mut BinaryReader) -> Self {
        Self {
            reader: Some(new_reader),
            values: self.values,
            kv_pairs: self.kv_pairs,
            system_metadata: self.system_metadata
        }
    }
}
