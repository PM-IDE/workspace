use std::rc::Rc;

use num_traits::FromPrimitive;
use rand::{distributions::Alphanumeric, rngs::ThreadRng, Rng};
use uuid::Uuid;

use bxes::models::domain::bxes_artifact::{BxesArtifact, BxesArtifactItem};
use bxes::models::domain::bxes_driver::{BxesDriver, BxesDrivers};
use bxes::models::domain::bxes_event_log::{BxesEvent, BxesEventLog, BxesTraceVariant};
use bxes::models::domain::bxes_lifecycle::{BrafLifecycle, Lifecycle, StandardLifecycle};
use bxes::models::domain::bxes_log_metadata::{
    BxesClassifier, BxesEventLogMetadata, BxesExtension, BxesGlobal, BxesGlobalKind,
};
use bxes::models::domain::bxes_value::BxesValue;
use bxes::models::domain::software_event_type::SoftwareEventType;
use bxes::models::system_models::{SystemMetadata, ValueAttributeDescriptor};
use bxes::type_ids::TypeIds;
use bxes::writer::writer_utils::BxesLogWriteData;

pub fn generate_random_bxes_write_data() -> BxesLogWriteData {
    let mut rng = rand::thread_rng();
    BxesLogWriteData {
        log: generate_random_log(&mut rng),
        system_metadata: generate_random_system_metadata(&mut rng),
    }
}

pub fn generate_random_system_metadata(rng: &mut ThreadRng) -> SystemMetadata {
    SystemMetadata {
        values_attrs: Some(generate_random_value_attributes_descriptor(rng)),
    }
}

fn generate_random_value_attributes_descriptor(
    rng: &mut ThreadRng,
) -> Vec<ValueAttributeDescriptor> {
    generate_random_list(rng, |rng| generate_random_value_attribute_descriptor(rng))
}

fn generate_random_value_attribute_descriptor(rng: &mut ThreadRng) -> ValueAttributeDescriptor {
    ValueAttributeDescriptor {
        type_id: generate_random_type_id(rng),
        name: generate_random_string(rng),
    }
}

pub fn generate_random_log(rng: &mut ThreadRng) -> BxesEventLog {
    BxesEventLog {
        version: rng.gen(),
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
        keys: generate_random_values(rng),
    })
}

fn generate_random_values(rng: &mut ThreadRng) -> Vec<Rc<Box<BxesValue>>> {
    generate_random_list(rng, |rng| generate_random_bxes_value(rng))
}

fn generate_random_list<T>(
    rng: &mut ThreadRng,
    item_generator: impl Fn(&mut ThreadRng) -> T,
) -> Vec<T> {
    let count = rng.gen_range(1..20);
    let mut vec = vec![];

    for _ in 0..count {
        vec.push(item_generator(rng));
    }

    vec
}

fn generate_random_variants(rng: &mut ThreadRng) -> Vec<BxesTraceVariant> {
    let variants_count = rng.gen_range(0..5);
    let mut variants = vec![];

    for _ in 0..variants_count {
        variants.push(generate_random_variant(rng));
    }

    variants
}

fn generate_random_variant(rng: &mut ThreadRng) -> BxesTraceVariant {
    let traces_count = rng.gen::<u32>();

    let mut metadata = vec![];
    let metadata_count = rng.gen_range(1..20);
    for _ in 0..metadata_count {
        metadata.push(generate_random_attribute(rng));
    }

    let mut events = vec![];

    let events_count = rng.gen_range(0..100);
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
        timestamp: rng.gen(),
        attributes: generate_random_attributes_option(rng),
    }
}

fn generate_random_attributes(
    rng: &mut ThreadRng,
) -> Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)> {
    generate_random_list(rng, |rng| generate_random_attribute(rng))
}

fn generate_random_attributes_option(
    rng: &mut ThreadRng,
) -> Option<Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>> {
    if rng.gen_bool(0.8) {
        Some(generate_random_attributes(rng))
    } else {
        None
    }
}

fn generate_random_attribute(rng: &mut ThreadRng) -> (Rc<Box<BxesValue>>, Rc<Box<BxesValue>>) {
    (
        generate_random_string_bxes_value(rng),
        generate_random_bxes_value(rng),
    )
}

fn generate_random_string_bxes_value(rng: &mut ThreadRng) -> Rc<Box<BxesValue>> {
    Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(
        generate_random_string(rng),
    )))))
}

fn generate_random_string(rng: &mut ThreadRng) -> String {
    let length = rng.gen_range(0..20);
    rng.sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn generate_random_bxes_value(rng: &mut ThreadRng) -> Rc<Box<BxesValue>> {
    Rc::new(Box::new(match generate_random_type_id(rng) {
        TypeIds::Null => BxesValue::Null,
        TypeIds::I32 => BxesValue::Int32(rng.gen()),
        TypeIds::I64 => BxesValue::Int64(rng.gen()),
        TypeIds::U32 => BxesValue::Uint32(rng.gen()),
        TypeIds::U64 => BxesValue::Uint64(rng.gen()),
        TypeIds::F32 => BxesValue::Float32(rng.gen()),
        TypeIds::F64 => BxesValue::Float64(rng.gen()),
        TypeIds::Bool => BxesValue::Bool(rng.gen()),
        TypeIds::String => BxesValue::String(Rc::new(Box::new(generate_random_string(rng)))),
        TypeIds::Timestamp => BxesValue::Timestamp(rng.gen()),
        TypeIds::BrafLifecycle => BxesValue::BrafLifecycle(generate_random_braf_lifecycle()),
        TypeIds::StandardLifecycle => {
            BxesValue::StandardLifecycle(generate_random_standard_lifecycle())
        }
        TypeIds::Guid => BxesValue::Guid(Uuid::new_v4()),
        TypeIds::SoftwareEventType => {
            BxesValue::SoftwareEventType(generate_random_enum::<SoftwareEventType>(
                SoftwareEventType::VARIANT_COUNT,
            ))
        }
        TypeIds::Artifact => generate_random_artifact(rng),
        TypeIds::Drivers => generate_random_drivers(rng),
        _ => panic!("Got unknown type id"),
    }))
}

fn generate_random_type_id(rng: &mut ThreadRng) -> TypeIds {
    TypeIds::from_u8(rng.gen_range(0..TypeIds::VARIANT_COUNT) as u8).unwrap()
}

fn generate_random_drivers(rng: &mut ThreadRng) -> BxesValue {
    let mut drivers = vec![];
    let count = rng.gen_range(1..20);

    for _ in 0..count {
        drivers.push(generate_random_driver(rng));
    }

    BxesValue::Drivers(BxesDrivers { drivers })
}

fn generate_random_driver(rng: &mut ThreadRng) -> BxesDriver {
    BxesDriver {
        amount: BxesValue::Float64(rng.gen()),
        name: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(
            generate_random_string(rng),
        ))))),
        driver_type: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(
            generate_random_string(rng),
        ))))),
    }
}

fn generate_random_artifact(rng: &mut ThreadRng) -> BxesValue {
    let mut artifacts = vec![];
    let count = rng.gen_range(1..20);

    for _ in 0..count {
        artifacts.push(generate_random_artifact_item(rng));
    }

    return BxesValue::Artifact(BxesArtifact { items: artifacts });
}

fn generate_random_artifact_item(rng: &mut ThreadRng) -> BxesArtifactItem {
    BxesArtifactItem {
        model: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(
            generate_random_string(rng),
        ))))),
        instance: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(
            generate_random_string(rng),
        ))))),
        transition: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(
            generate_random_string(rng),
        ))))),
    }
}

fn generate_random_lifecycle(rng: &mut ThreadRng) -> Lifecycle {
    match rng.gen_bool(0.5) {
        true => Lifecycle::Standard(generate_random_standard_lifecycle()),
        false => Lifecycle::Braf(generate_random_braf_lifecycle()),
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
