use chrono::Utc;
use ficus::event_log::core::event::event::EventPayloadValue;
use ficus::event_log::xes::xes_event::XesEventImpl;
use ficus::features::discovery::timeline::software_data::extraction_config::{AllocationExtractionConfig, ExtractionConfig, MethodCommonAttributesConfig, MethodInliningConfig, MethodInliningFailedConfig, MethodInliningSucceededConfig, SoftwareDataExtractionConfig};
use ficus::features::discovery::timeline::software_data::extractors::allocations::AllocationDataExtractor;
use ficus::features::discovery::timeline::software_data::extractors::core::SoftwareDataExtractor;
use ficus::features::discovery::timeline::software_data::models::SoftwareData;
use std::cell::RefCell;
use std::rc::Rc;
use ficus::features::discovery::timeline::software_data::extractors::methods::MethodsDataExtractor;

#[test]
fn test_allocations_extraction_1() {
  execute_test_with_software_data(
    r#"{"allocation_events":[{"type_name":"xd","objects_count":32,"allocated_bytes":1234}]}"#,
    || {
      let events = [
        create_event_with_attributes("Alloc".to_string(), vec![
          ("count".to_string(), EventPayloadValue::Int32(32)),
          ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd".to_string())))),
          ("total".to_string(), EventPayloadValue::Int64(1234)),
        ]),
        create_event_with_attributes("All1oc".to_string(), vec![
          ("count".to_string(), EventPayloadValue::Int32(32)),
          ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd".to_string())))),
          ("total".to_string(), EventPayloadValue::Int64(1234)),
        ])
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      let alloc_config = AllocationExtractionConfig::new("type".to_string(), "count".to_string(), None, Some("total".to_string()));
      config.set_allocation(Some(ExtractionConfig::new("Alloc".to_string(), alloc_config)));

      let extractor = AllocationDataExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data
    },
  )
}

#[test]
fn test_allocations_extraction_2() {
  execute_test_with_software_data(
    r#"{"allocation_events":[{"type_name":"xd","objects_count":32,"allocated_bytes":39488}]}"#,
    || {
      let events = [
        create_event_with_attributes("Alloc".to_string(), vec![
          ("count".to_string(), EventPayloadValue::Int32(32)),
          ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd".to_string())))),
          ("total".to_string(), EventPayloadValue::Int64(1234)),
        ]),
        create_event_with_attributes("Al123loc".to_string(), vec![
          ("count".to_string(), EventPayloadValue::Int32(32)),
          ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd".to_string())))),
          ("total".to_string(), EventPayloadValue::Int64(1234)),
        ])
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      let alloc_config = AllocationExtractionConfig::new("type".to_string(), "count".to_string(), Some("total".to_string()), None);
      config.set_allocation(Some(ExtractionConfig::new("Alloc".to_string(), alloc_config)));

      let extractor = AllocationDataExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data
    },
  )
}

#[test]
fn test_inlining_success_extraction() {
  execute_test_with_software_data(
    r#"{"method_inlinings_events":[{"InliningSuccess":{"inlinee_info":{"name":"xd1","namespace":"xd2","signature":"xd3"},"inliner_info":{"name":"xd4","namespace":"xd5","signature":"xd6"}}}]}"#,
    || {
      let events = [
        create_event_with_attributes("Inlining".to_string(), vec![
          ("m1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd1".to_string())))),
          ("n1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd2".to_string())))),
          ("s1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd3".to_string())))),
          ("m2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd4".to_string())))),
          ("n2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd5".to_string())))),
          ("s2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd6".to_string())))),
        ]),
        create_event_with_attributes("Inli123ning".to_string(), vec![
          ("m1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd1".to_string())))),
          ("n1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd2".to_string())))),
          ("s1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd3".to_string())))),
          ("m2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd4".to_string())))),
          ("n2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd5".to_string())))),
          ("s2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd6".to_string())))),
        ]),
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      let inlining_success_config = MethodInliningSucceededConfig::new(MethodInliningConfig::new(
        MethodCommonAttributesConfig::new("m1".to_string(), "n1".to_string(), "s1".to_string()),
        MethodCommonAttributesConfig::new("m2".to_string(), "n2".to_string(), "s2".to_string()),
      ));

      config.set_method_inlining_success(Some(ExtractionConfig::new("Inlining".to_string(), inlining_success_config)));

      let extractor = MethodsDataExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data
    },
  )
}

#[test]
fn test_inlining_failed_extraction() {
  execute_test_with_software_data(
    r#"{"method_inlinings_events":[{"InliningFailed":[{"inlinee_info":{"name":"xd1","namespace":"xd2","signature":"xd3"},"inliner_info":{"name":"xd4","namespace":"xd5","signature":"xd6"}},"failed"]}]}"#,
    || {
      let events = [
        create_event_with_attributes("Inlining".to_string(), vec![
          ("m1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd1".to_string())))),
          ("n1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd2".to_string())))),
          ("s1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd3".to_string())))),
          ("m2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd4".to_string())))),
          ("n2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd5".to_string())))),
          ("s2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd6".to_string())))),
          ("reason".to_string(), EventPayloadValue::String(Rc::new(Box::new("failed".to_string()))))
        ]),
        create_event_with_attributes("Inli123ning".to_string(), vec![
          ("m1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd1".to_string())))),
          ("n1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd2".to_string())))),
          ("s1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd3".to_string())))),
          ("m2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd4".to_string())))),
          ("n2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd5".to_string())))),
          ("s2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd6".to_string())))),
          ("reason".to_string(), EventPayloadValue::String(Rc::new(Box::new("failed".to_string()))))
        ]),
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      let inlining_success_config = MethodInliningFailedConfig::new(
        MethodInliningConfig::new(
          MethodCommonAttributesConfig::new("m1".to_string(), "n1".to_string(), "s1".to_string()),
          MethodCommonAttributesConfig::new("m2".to_string(), "n2".to_string(), "s2".to_string()),
        ),
        "reason".to_string(),
      );

      config.set_method_inlining_failed(Some(ExtractionConfig::new("Inlining".to_string(), inlining_success_config)));

      let extractor = MethodsDataExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data
    },
  )
}

fn create_event_with_attributes(name: String, attributes: Vec<(String, EventPayloadValue)>) -> Rc<RefCell<XesEventImpl>> {
  Rc::new(RefCell::new(XesEventImpl::new_all_fields(Rc::new(Box::new(name)), Utc::now(), Some(attributes.into_iter().collect()))))
}

fn execute_test_with_software_data(gold: &str, test: impl Fn() -> SoftwareData) {
  assert_eq!(gold, serde_json::to_string(&test()).unwrap());
}