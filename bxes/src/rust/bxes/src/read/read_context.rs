use crate::binary_rw::core::BinaryReader;
use crate::models::domain::bxes_value::BxesValue;
use crate::models::system_models::SystemMetadata;
use std::rc::Rc;

pub struct ReadMetadata {
    pub values: Option<Vec<Rc<Box<BxesValue>>>>,
    pub kv_pairs: Option<Vec<(u32, u32)>>,
    pub system_metadata: Option<SystemMetadata>,
}

impl ReadMetadata {
    pub fn empty() -> Self {
        Self {
            values: None,
            kv_pairs: None,
            system_metadata: None,
        }
    }
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
        Self {
            reader: None,
            metadata,
        }
    }

    pub fn set_reader(&mut self, reader: &'a mut BinaryReader<'a>) {
        self.reader = Some(reader);
    }
}
