use crate::{
  event_log::{
    core::event::event::{Event, EventPayloadValue},
    xes::xes_event::XesEventImpl,
  },
  features::discovery::timeline::software_data::{
    extraction_config::{
      ExtractionConfig, OcelAllocateMergeExtractionConfig, OcelConsumeProduceExtractionConfig, OcelObjectExtractionConfigBase,
      SoftwareDataExtractionConfig,
    },
    extractors::core::{EventGroupSoftwareDataExtractor, SoftwareDataExtractionError},
    models::{ObjectTypeWithData, OcelData, OcelObjectAction, OcelProducedObjectAfterConsume, SoftwareData},
  },
};
use derive_new::new;
use fancy_regex::Regex;
use log::{debug, warn};
use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

#[derive(Debug, Clone, new)]
pub struct OcelDataExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig,
}

impl<'a> EventGroupSoftwareDataExtractor for OcelDataExtractor<'a> {
  fn extract_from_events(
    &self,
    software_data: &mut SoftwareData,
    events: &[Rc<RefCell<XesEventImpl>>],
  ) -> Result<(), SoftwareDataExtractionError> {
    let Some(ocel_config) = self.config.ocel().as_ref() else {
      return Ok(());
    };

    let alloc_config = Self::map_config_to_regex(ocel_config.allocated())?;
    let consume_config = Self::map_config_to_regex(ocel_config.consumed())?;
    let alloc_merged_config = Self::map_config_to_regex(ocel_config.allocated_merged())?;
    let consume_produce_config = Self::map_config_to_regex(ocel_config.consume_produce())?;

    let delimiter = ocel_config.delimiter().as_ref().map(|s| s.as_str()).unwrap_or(" ");

    for event in events {
      let event = &event.borrow();

      let _ = Self::process_allocate_merge(event, alloc_merged_config.as_ref(), delimiter, software_data)
        || Self::process_consume_produce(event, consume_produce_config.as_ref(), delimiter, software_data)
        || Self::process_allocate(event, alloc_config.as_ref(), software_data)
        || Self::process_consume(event, consume_config.as_ref(), software_data);
    }

    Ok(())
  }
}

impl<'a> OcelDataExtractor<'a> {
  fn map_config_to_regex<T: Clone + Debug>(
    config: &Option<ExtractionConfig<T>>,
  ) -> Result<Option<(Regex, &T)>, SoftwareDataExtractionError> {
    let Some(config) = config else { return Ok(None) };

    let regex = config.event_class_regex();
    let regex = Regex::new(regex).map_err(|_| SoftwareDataExtractionError::FailedToParseRegex(regex.to_owned()))?;

    Ok(Some((regex, config.info())))
  }

  fn process_allocate(
    event: &XesEventImpl,
    config: Option<&(Regex, &OcelObjectExtractionConfigBase)>,
    software_data: &mut SoftwareData,
  ) -> bool {
    let Some(config) = Self::try_get_config(event, config) else {
      return false;
    };
    let Some((id, obj_type)) = Self::extract_object_id_and_type(event, config) else {
      return false;
    };
    let action = OcelObjectAction::Allocate(ObjectTypeWithData::new(Some(obj_type), ()));

    software_data.ocel_data_mut().push(OcelData::new(id, action));

    true
  }

  fn try_get_config<'b, T>(event: &'b XesEventImpl, config: Option<&'b (Regex, T)>) -> Option<&'b T> {
    let (regex, config) = config?;

    if !regex.is_match(event.name()).unwrap_or(false) {
      return None;
    }

    Some(config)
  }

  fn process_consume(
    event: &XesEventImpl,
    config: Option<&(Regex, &OcelObjectExtractionConfigBase)>,
    software_data: &mut SoftwareData,
  ) -> bool {
    let Some(config) = Self::try_get_config(event, config) else {
      return false;
    };

    let Some((id, obj_type)) = Self::extract_object_id_and_type(event, config) else {
      return false;
    };
    let action = OcelObjectAction::Consume(ObjectTypeWithData::new(Some(obj_type), ()));

    software_data.ocel_data_mut().push(OcelData::new(id, action));

    true
  }

  fn extract_object_id_and_type(event: &XesEventImpl, config: &OcelObjectExtractionConfigBase) -> Option<(Rc<str>, Rc<str>)> {
    let object_type = config.object_type_attr().create(event);

    let object_id = match Self::parse_object_id(event, config.object_id_attr().as_str()) {
      None => {
        debug!("Object does not have an object id, skipping it");
        return None;
      }
      Some(id) => id,
    };

    Some((object_id, object_type))
  }

  fn process_allocate_merge(
    event: &XesEventImpl,
    config: Option<&(Regex, &OcelAllocateMergeExtractionConfig)>,
    delimiter: &str,
    software_data: &mut SoftwareData,
  ) -> bool {
    let Some(config) = Self::try_get_config(event, config) else {
      return false;
    };

    let Some(payload) = event.payload_map() else { return false };
    let Some((id, obj_type)) = Self::extract_object_id_and_type(event, config.allocated_obj()) else {
      return false;
    };

    let related_objects_ids = Self::parse_related_objects_ids(payload, Some(config.related_object_ids_attr()), delimiter);
    let Some(related_objects_ids) = related_objects_ids else {
      return false;
    };

    let data = ObjectTypeWithData::new(Some(obj_type), related_objects_ids);
    let ocel_data = OcelData::new(id, OcelObjectAction::AllocateMerged(data));
    software_data.ocel_data_mut().push(ocel_data);

    true
  }

  fn process_consume_produce(
    event: &XesEventImpl,
    config: Option<&(Regex, &OcelConsumeProduceExtractionConfig)>,
    delimiter: &str,
    software_data: &mut SoftwareData,
  ) -> bool {
    let Some(config) = Self::try_get_config(event, config) else {
      return false;
    };

    let Some(payload) = event.payload_map() else { return false };
    let Some(object_id) = Self::parse_object_id(event, config.object_id_attr().as_ref()) else {
      return false;
    };

    let related_objects_ids = Self::parse_related_objects_ids(payload, Some(config.related_object_ids_attr()), delimiter);
    let Some(related_objects_ids) = related_objects_ids else {
      return false;
    };

    let related_objects_types = Self::parse_related_objects_ids(payload, Some(config.related_object_type_attr()), delimiter);
    let Some(related_objects_types) = related_objects_types else {
      return false;
    };

    if related_objects_ids.len() != related_objects_types.len() {
      warn!("related_objects_ids.len() != related_objects_types.len(), will not add consume produce");
      return false;
    }

    let data = related_objects_ids
      .into_iter()
      .zip(related_objects_types)
      .map(|(id, r#type)| OcelProducedObjectAfterConsume::new(id, Some(r#type)))
      .collect();

    let ocel_data = OcelData::new(object_id, OcelObjectAction::ConsumeWithProduce(data));
    software_data.ocel_data_mut().push(ocel_data);

    true
  }

  fn parse_object_id(event: &XesEventImpl, object_id_attr: &str) -> Option<Rc<str>> {
    if let Some(map) = event.payload_map().as_ref()
      && let Some(object_id) = map.get(object_id_attr).as_ref()
    {
      return Some(object_id.to_string_repr());
    }

    None
  }

  fn parse_related_objects_ids(
    payload: &HashMap<Rc<str>, EventPayloadValue>,
    related_objects_ids_attr: Option<&Rc<str>>,
    delimiter: &str,
  ) -> Option<Vec<Rc<str>>> {
    if let Some(related_objects_ids_attr) = related_objects_ids_attr
      && let Some(objects_ids) = payload.get(related_objects_ids_attr)
    {
      let parsed_ids: Vec<Rc<str>> = objects_ids
        .to_string_repr()
        .trim()
        .split(delimiter)
        .filter_map(|s| if !s.is_empty() { Some(Rc::from(s)) } else { None })
        .collect();

      if parsed_ids.is_empty() {
        return None;
      }

      return Some(parsed_ids);
    }

    None
  }
}
