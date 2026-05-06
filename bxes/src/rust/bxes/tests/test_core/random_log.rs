use bxes::read::read_utils::string_or_err;
use num_traits::FromPrimitive;
use rand::{Rng, RngExt, distr::Alphanumeric, rngs::ThreadRng};
use std::sync::Arc;
use std::{collections::HashMap, rc::Rc};
use uuid::Uuid;

use bxes::{
  models::{
    domain::{
      bxes_artifact::{BxesArtifact, BxesArtifactItem},
      bxes_driver::{BxesDriver, BxesDrivers},
      bxes_event_log::{BxesEvent, BxesEventLog, BxesTraceVariant},
      bxes_lifecycle::{BrafLifecycle, StandardLifecycle},
      bxes_log_metadata::{BxesClassifier, BxesEventLogMetadata, BxesExtension, BxesGlobal, BxesGlobalKind},
      bxes_value::BxesValue,
      software_event_type::SoftwareEventType,
      type_ids::{TypeIds, get_type_id},
    },
    system_models::{SystemMetadata, ValueAttributeDescriptor},
  },
  writer::writer_utils::BxesLogWriteData,
};

pub fn generate_random_bxes_write_data() -> BxesLogWriteData {
  let mut rng = rand::rng();
  let log = generate_random_log(&mut rng);
  let system_metadata = generate_random_system_metadata(&mut rng, &log);

  BxesLogWriteData { log, system_metadata }
}

pub fn generate_random_system_metadata(rng: &mut ThreadRng, log: &BxesEventLog) -> SystemMetadata {
  let mut descriptors = HashMap::new();
  let count = rng.random_range(50..100);

  let mut index = 0;
  loop {
    if index == count {
      break;
    }

    let random_variant = log.variants.get(rng.random_range(0..log.variants.len())).unwrap();

    let random_event = random_variant.events.get(rng.random_range(0..random_variant.events.len())).unwrap();

    if let Some(attrs) = random_event.attributes.as_ref() {
      if attrs.len() == 0 {
        continue;
      }

      let random_attr = attrs.get(rng.random_range(0..attrs.len())).unwrap();
      let key = string_or_err(&random_attr.0).ok().unwrap();
      if descriptors.contains_key(&key) {
        continue;
      }

      descriptors.insert(key, get_type_id(random_attr.1.as_ref()));
      index += 1;
    }
  }

  SystemMetadata {
    values_attrs: Some(
      descriptors
        .iter()
        .map(|pair| ValueAttributeDescriptor {
          name: pair.0.clone(),
          type_id: pair.1.clone(),
        })
        .collect(),
    ),
  }
}

pub fn generate_random_log(rng: &mut ThreadRng) -> BxesEventLog {
  BxesEventLog {
    version: rng.next_u32(),
    metadata: generate_random_metadata(rng),
    variants: generate_random_variants(rng),
  }
}

fn generate_random_metadata(rng: &mut ThreadRng) -> BxesEventLogMetadata {
  BxesEventLogMetadata {
    extensions: Some(generate_random_extensions(rng)),
    classifiers: Some(generate_random_classifiers(rng)),
    properties: generate_random_attributes_option(rng),
    globals: Some(generate_random_globals(rng)),
  }
}

fn generate_random_globals(rng: &mut ThreadRng) -> Vec<BxesGlobal> {
  generate_random_list(rng, |rng| BxesGlobal {
    entity_kind: generate_random_enum::<BxesGlobalKind>(BxesGlobalKind::VARIANT_COUNT),
    globals: generate_random_attributes(rng),
  })
}

fn generate_random_extensions(rng: &mut ThreadRng) -> Vec<BxesExtension> {
  generate_random_list(rng, |rng| BxesExtension {
    name: generate_random_string_bxes_value(rng),
    prefix: generate_random_string_bxes_value(rng),
    uri: generate_random_string_bxes_value(rng),
  })
}

fn generate_random_classifiers(rng: &mut ThreadRng) -> Vec<BxesClassifier> {
  generate_random_list(rng, |rng| BxesClassifier {
    name: generate_random_string_bxes_value(rng),
    keys: vec![],
  })
}

fn generate_random_list<T>(rng: &mut ThreadRng, item_generator: impl Fn(&mut ThreadRng) -> T) -> Vec<T> {
  let count = rng.random_range(100..500);
  let mut vec = vec![];

  for _ in 0..count {
    vec.push(item_generator(rng));
  }

  vec
}

fn generate_random_variants(rng: &mut ThreadRng) -> Vec<BxesTraceVariant> {
  let variants_count = rng.random_range(1..5);
  let mut variants = vec![];

  for _ in 0..variants_count {
    variants.push(generate_random_variant(rng));
  }

  variants
}

fn generate_random_variant(rng: &mut ThreadRng) -> BxesTraceVariant {
  let traces_count = rng.random();

  let mut metadata = vec![];
  let metadata_count = rng.random_range(1..20);
  for _ in 0..metadata_count {
    metadata.push(generate_random_attribute(rng));
  }

  let mut events = vec![];

  let events_count = rng.random_range(50..100);
  for _ in 0..events_count {
    events.push(generate_random_event(rng));
  }

  BxesTraceVariant {
    traces_count,
    metadata,
    events,
  }
}

fn generate_random_event(rng: &mut ThreadRng) -> BxesEvent {
  BxesEvent {
    name: generate_random_string_bxes_value(rng),
    timestamp: rng.random(),
    attributes: generate_random_attributes_option(rng),
  }
}

fn generate_random_attributes(rng: &mut ThreadRng) -> Vec<(Arc<BxesValue>, Arc<BxesValue>)> {
  generate_random_list(rng, |rng| generate_random_attribute(rng))
}

fn generate_random_attributes_option(rng: &mut ThreadRng) -> Option<Vec<(Arc<BxesValue>, Arc<BxesValue>)>> {
  if rng.random_bool(0.8) {
    Some(generate_random_attributes(rng))
  } else {
    None
  }
}

fn generate_random_attribute(rng: &mut ThreadRng) -> (Arc<BxesValue>, Arc<BxesValue>) {
  (generate_random_string_bxes_value(rng), generate_random_bxes_value(rng))
}

fn generate_random_string_bxes_value(rng: &mut ThreadRng) -> Arc<BxesValue> {
  Arc::new(BxesValue::String(Arc::from(generate_random_string(rng))))
}

fn generate_random_string(rng: &mut ThreadRng) -> String {
  let length = rng.random_range(50..100);
  rng.sample_iter(&Alphanumeric).take(length).map(char::from).collect()
}

fn generate_random_bxes_value(rng: &mut ThreadRng) -> Arc<BxesValue> {
  Arc::new(match generate_random_type_id(rng) {
    TypeIds::Null => BxesValue::Int32(rng.random()),
    TypeIds::I32 => BxesValue::Int32(rng.random()),
    TypeIds::I64 => BxesValue::Int64(rng.random()),
    TypeIds::U32 => BxesValue::Uint32(rng.random()),
    TypeIds::U64 => BxesValue::Uint64(rng.random()),
    TypeIds::F32 => BxesValue::Float32(rng.random()),
    TypeIds::F64 => BxesValue::Float64(rng.random()),
    TypeIds::Bool => BxesValue::Bool(rng.random()),
    TypeIds::String => BxesValue::String(Arc::from(generate_random_string(rng))),
    TypeIds::Timestamp => BxesValue::Timestamp(rng.random()),
    TypeIds::BrafLifecycle => BxesValue::BrafLifecycle(generate_random_braf_lifecycle()),
    TypeIds::StandardLifecycle => BxesValue::StandardLifecycle(generate_random_standard_lifecycle()),
    TypeIds::Guid => BxesValue::Guid(Uuid::new_v4()),
    TypeIds::SoftwareEventType => BxesValue::SoftwareEventType(generate_random_enum::<SoftwareEventType>(SoftwareEventType::VARIANT_COUNT)),
    TypeIds::Artifact => generate_random_artifact(rng),
    TypeIds::Drivers => generate_random_drivers(rng),
  })
}

fn generate_random_type_id(rng: &mut ThreadRng) -> TypeIds {
  TypeIds::from_u8(rng.random_range(0..TypeIds::VARIANT_COUNT) as u8).unwrap()
}

fn generate_random_drivers(rng: &mut ThreadRng) -> BxesValue {
  let mut drivers = vec![];
  let count = rng.random_range(1..20);

  for _ in 0..count {
    drivers.push(generate_random_driver(rng));
  }

  BxesValue::Drivers(BxesDrivers { drivers })
}

fn generate_random_driver(rng: &mut ThreadRng) -> BxesDriver {
  BxesDriver {
    amount: BxesValue::Float64(rng.random()),
    name: Arc::new(BxesValue::String(Arc::from(generate_random_string(rng)))),
    driver_type: Arc::new(BxesValue::String(Arc::from(generate_random_string(rng)))),
  }
}

fn generate_random_artifact(rng: &mut ThreadRng) -> BxesValue {
  let mut artifacts = vec![];
  let count = rng.random_range(1..20);

  for _ in 0..count {
    artifacts.push(generate_random_artifact_item(rng));
  }

  BxesValue::Artifact(BxesArtifact { items: artifacts })
}

fn generate_random_artifact_item(rng: &mut ThreadRng) -> BxesArtifactItem {
  BxesArtifactItem {
    model: Arc::new(BxesValue::String(Arc::from(generate_random_string(rng)))),
    instance: Arc::new(BxesValue::String(Arc::from(generate_random_string(rng)))),
    transition: Arc::new(BxesValue::String(Arc::from(generate_random_string(rng)))),
  }
}

fn generate_random_braf_lifecycle() -> BrafLifecycle {
  generate_random_enum::<BrafLifecycle>(BrafLifecycle::VARIANT_COUNT)
}

fn generate_random_enum<T: FromPrimitive>(variant_count: usize) -> T {
  T::from_usize(variant_count - 1).unwrap()
}

fn generate_random_standard_lifecycle() -> StandardLifecycle {
  generate_random_enum::<StandardLifecycle>(StandardLifecycle::VARIANT_COUNT)
}
