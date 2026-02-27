use chrono::Utc;
use ficus::{
  event_log::{core::event::event::EventPayloadValue, xes::xes_event::XesEventImpl},
  features::discovery::timeline::{
    events_groups::EventGroup,
    software_data::{
      extraction_config::{
        ActivityDurationExtractionConfig, ExtractionConfig, GenericExtractionConfigBase, NameCreationStrategy,
        OcelAllocateMergeExtractionConfig, OcelConsumeProduceExtractionConfig, OcelObjectExtractionConfigBase, OcelUnitedExtractionConfig,
        PieChartExtractionConfig, SimpleCountExtractionConfig, SingleAttribute, SoftwareDataExtractionConfig, TimeAttributeConfig,
        TimeKind,
      },
      extractors::{
        activities_durations::ActivityDurationExtractor,
        core::{EventGroupSoftwareDataExtractor, EventGroupTraceSoftwareDataExtractor},
        ocel::OcelDataExtractor,
        pie_charts::PieChartExtractor,
        simple_counter::SimpleCounterExtractor,
      },
      models::SoftwareData,
    },
  },
  utils::references::heaped,
};
use std::{cell::RefCell, rc::Rc};

#[test]
fn test_general_histogram() {
  execute_test_with_software_data(
    r#"{"histograms":[{"base":{"name":"g1","units":"units","group":null},"entries":[{"name":"type1","value":246.0},{"name":"type2","value":123.0}]},{"base":{"name":"g2","units":"units","group":null},"entries":[{"name":"type1","value":123.0},{"name":"type2","value":123.0}]}]}"#,
    || {
      let events = [
        create_event_with_attributes(
          "histogram_event".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new("type1".to_string())),
            ),
            ("count".to_string(), EventPayloadValue::Float64(123.)),
          ],
        ),
        create_event_with_attributes(
          "histogram_event".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new("type2".to_string())),
            ),
            ("count".to_string(), EventPayloadValue::Float32(123.)),
          ],
        ),
        create_event_with_attributes(
          "histogram_event".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new("type1".to_string())),
            ),
            ("count".to_string(), EventPayloadValue::Uint64(123)),
          ],
        ),
        create_event_with_attributes(
          "unknown".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new("type1".to_string())),
            ),
            ("count".to_string(), EventPayloadValue::Uint32(123)),
          ],
        ),
        create_event_with_attributes(
          "hst_event".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new("type1".to_string())),
            ),
            ("count".to_string(), EventPayloadValue::Int64(123)),
          ],
        ),
        create_event_with_attributes(
          "hst_event".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new("type2".to_string())),
            ),
            ("count".to_string(), EventPayloadValue::Int32(123)),
          ],
        ),
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      config.set_pie_chart_extraction_configs(vec![
        ExtractionConfig::new(
          "histogram_event".to_string(),
          PieChartExtractionConfig::new(
            GenericExtractionConfigBase::new(heaped("g1".to_string()), heaped("units".to_string()), None),
            Some(NameCreationStrategy::SingleAttribute(SingleAttribute::new(
              "type".to_string(),
              heaped("xd".to_string()),
            ))),
            Some("count".to_string()),
          ),
        ),
        ExtractionConfig::new(
          "hst_event".to_string(),
          PieChartExtractionConfig::new(
            GenericExtractionConfigBase::new(heaped("g2".to_string()), heaped("units".to_string()), None),
            Some(NameCreationStrategy::SingleAttribute(SingleAttribute::new(
              "type".to_string(),
              heaped("xd".to_string()),
            ))),
            Some("count".to_string()),
          ),
        ),
      ]);

      let extractor = PieChartExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data.histograms_mut().sort_by(|f, s| f.base().name().cmp(s.base().name()));
      software_data
        .histograms_mut()
        .iter_mut()
        .for_each(|counts| counts.entries_mut().sort_by(|f, s| f.name().cmp(s.name())));

      software_data
    },
  )
}

#[test]
fn test_simple_counter() {
  execute_test_with_software_data(
    r#"{"simple_counters":[{"base":{"name":"counter1","units":"units","group":null},"value":3.0},{"base":{"name":"counter2","units":"units","group":null},"value":246.0}]}"#,
    || {
      let events = [
        create_event_with_attributes(
          "histogram_event".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new("type1".to_string())),
            ),
            ("count".to_string(), EventPayloadValue::Float64(123.)),
          ],
        ),
        create_event_with_attributes(
          "histogram_event".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new("type2".to_string())),
            ),
            ("count".to_string(), EventPayloadValue::Float32(123.)),
          ],
        ),
        create_event_with_attributes(
          "histogram_event".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new("type1".to_string())),
            ),
            ("count".to_string(), EventPayloadValue::Uint64(123)),
          ],
        ),
        create_event_with_attributes(
          "unknown".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new("type1".to_string())),
            ),
            ("count".to_string(), EventPayloadValue::Uint32(123)),
          ],
        ),
        create_event_with_attributes(
          "hst_event".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new("type1".to_string())),
            ),
            ("count".to_string(), EventPayloadValue::Int64(123)),
          ],
        ),
        create_event_with_attributes(
          "hst_event".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new("type2".to_string())),
            ),
            ("count".to_string(), EventPayloadValue::Int32(123)),
          ],
        ),
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      config.set_simple_counter_configs(vec![
        ExtractionConfig::new(
          "histogram_event".to_string(),
          SimpleCountExtractionConfig::new(
            GenericExtractionConfigBase::new(heaped("counter1".to_string()), heaped("units".to_string()), None),
            None,
          ),
        ),
        ExtractionConfig::new(
          "hst_event".to_string(),
          SimpleCountExtractionConfig::new(
            GenericExtractionConfigBase::new(heaped("counter2".to_string()), heaped("units".to_string()), None),
            Some("count".to_string()),
          ),
        ),
      ]);

      let extractor = SimpleCounterExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data
        .simple_counters_mut()
        .sort_by(|f, s| f.base().name().cmp(s.base().name()));
      software_data
    },
  )
}

#[test]
fn test_activities_duration() {
  execute_test_with_activities_durations(
    r#"[[{"activities_durations":[{"base":{"name":"activity","units":"units","group":null},"duration":300,"kind":"Unknown"}]},{"activities_durations":[{"base":{"name":"activity","units":"units","group":null},"duration":50,"kind":"Unknown"}]}]]"#,
    vec![vec![
      vec![
        create_event_with_attributes("some_event".to_string(), vec![("stamp".to_string(), EventPayloadValue::Int64(50))]),
        create_event_with_attributes("some_event".to_string(), vec![("stamp".to_string(), EventPayloadValue::Int64(250))]),
      ],
      vec![
        create_event_with_attributes(
          "event_start".to_string(),
          vec![
            (
              "activity_id".to_string(),
              EventPayloadValue::String(Rc::new("1".to_string())),
            ),
            ("stamp".to_string(), EventPayloadValue::Int64(100)),
          ],
        ),
        create_event_with_attributes(
          "event_end".to_string(),
          vec![
            (
              "activity_id".to_string(),
              EventPayloadValue::String(Rc::new("1".to_string())),
            ),
            ("stamp".to_string(), EventPayloadValue::Int64(200)),
          ],
        ),
      ],
      vec![create_event_with_attributes(
        "event_end".to_string(),
        vec![
          (
            "activity_id".to_string(),
            EventPayloadValue::String(Rc::new("2".to_string())),
          ),
          ("stamp".to_string(), EventPayloadValue::Int64(300)),
        ],
      )],
    ]],
  );
}

fn execute_test_with_activities_durations(gold: &str, raw_event_groups: Vec<Vec<Vec<Rc<RefCell<XesEventImpl>>>>>) {
  let mut config = SoftwareDataExtractionConfig::empty();
  config.set_activities_duration_configs(vec![ActivityDurationExtractionConfig::new(
    GenericExtractionConfigBase::new(heaped("activity".to_string()), heaped("units".to_string()), None),
    "event_start".to_string(),
    "event_end".to_string(),
    Some(TimeAttributeConfig::new("stamp".to_string(), TimeKind::Unknown)),
    Some(NameCreationStrategy::SingleAttribute(SingleAttribute::new(
      "activity_id".to_string(),
      heaped("xd".to_string()),
    ))),
  )]);

  let event_groups: Vec<EventGroup> = raw_event_groups
    .into_iter()
    .map(|x| {
      let mut group = EventGroup::empty();

      group.control_flow_events_mut().extend(x[0].clone());
      group.statistic_events_mut().extend(x[1].clone());
      group.set_after_group_events(if x[2].is_empty() { None } else { Some(x[2].clone()) });

      group
    })
    .collect();

  let extractor = ActivityDurationExtractor::new(&config);
  let mut software_data = event_groups
    .iter()
    .map(|_| (SoftwareData::empty(), SoftwareData::empty()))
    .collect();

  extractor.extract(&event_groups, &mut software_data).ok().unwrap();

  let test = serde_json::to_string(&software_data).unwrap();

  if gold != test {
    println!("Test value: {}", test);
    assert!(false);
  }
}

#[test]
fn test_activities_duration_2() {
  execute_test_with_activities_durations(
    r#"[[{"activities_durations":[{"base":{"name":"activity","units":"units","group":null},"duration":50,"kind":"Unknown"}]},{}],[{"activities_durations":[{"base":{"name":"activity","units":"units","group":null},"duration":100,"kind":"Unknown"}]},{"activities_durations":[{"base":{"name":"activity","units":"units","group":null},"duration":300,"kind":"Unknown"}]}]]"#,
    vec![
      vec![
        vec![
          create_event_with_attributes("some_event".to_string(), vec![("stamp".to_string(), EventPayloadValue::Int64(50))]),
          create_event_with_attributes("some_event".to_string(), vec![("stamp".to_string(), EventPayloadValue::Int64(100))]),
        ],
        vec![create_event_with_attributes(
          "event_start".to_string(),
          vec![
            (
              "activity_id".to_string(),
              EventPayloadValue::String(Rc::new("2".to_string())),
            ),
            ("stamp".to_string(), EventPayloadValue::Int64(50)),
          ],
        )],
        vec![],
      ],
      vec![
        vec![
          create_event_with_attributes("some_event".to_string(), vec![("stamp".to_string(), EventPayloadValue::Int64(100))]),
          create_event_with_attributes("some_event".to_string(), vec![("stamp".to_string(), EventPayloadValue::Int64(200))]),
        ],
        vec![],
        vec![create_event_with_attributes(
          "some_event".to_string(),
          vec![("stamp".to_string(), EventPayloadValue::Int64(500))],
        )],
      ],
    ],
  );
}

#[test]
pub fn test_ocel_data_extraction() {
  execute_test_with_software_data(
    r#"{"ocel_data":[{"object_id":"id_1","action":{"Allocate":{"type":"type1","data":null}}},{"object_id":"id_2","action":{"Consume":{"type":"type1","data":null}}},{"object_id":"id_3","action":{"Allocate":{"type":"type1","data":null}}},{"object_id":"id_2","action":{"ConsumeWithProduce":[{"id":"1","type":"T1"},{"id":"2","type":"T2"},{"id":"3","type":"T3"},{"id":"4","type":"T4"},{"id":"5","type":"T5"}]}},{"object_id":"id_2","action":{"AllocateMerged":{"type":"type1","data":["1","2","3","4","5"]}}}]}"#,
    || {
      let events = [
        create_event_with_attributes(
          "ocel_allocate".to_string(),
          vec![
            (
              "object_type".to_string(),
              EventPayloadValue::String(Rc::new("type1".to_string())),
            ),
            (
              "object_id".to_string(),
              EventPayloadValue::String(Rc::new("id_1".to_string())),
            ),
          ],
        ),
        create_event_with_attributes(
          "ocel_consume".to_string(),
          vec![
            (
              "object_type".to_string(),
              EventPayloadValue::String(Rc::new("type1".to_string())),
            ),
            (
              "object_id".to_string(),
              EventPayloadValue::String(Rc::new("id_2".to_string())),
            ),
          ],
        ),
        create_event_with_attributes(
          "ocel_allocate".to_string(),
          vec![
            (
              "object_type".to_string(),
              EventPayloadValue::String(Rc::new("type1".to_string())),
            ),
            (
              "object_id".to_string(),
              EventPayloadValue::String(Rc::new("id_3".to_string())),
            ),
          ],
        ),
        create_event_with_attributes(
          "unknown".to_string(),
          vec![
            (
              "object_type".to_string(),
              EventPayloadValue::String(Rc::new("type1".to_string())),
            ),
            (
              "object_id".to_string(),
              EventPayloadValue::String(Rc::new("id_2123123".to_string())),
            ),
          ],
        ),
        create_event_with_attributes(
          "ocel_consume_produce".to_string(),
          vec![
            (
              "object_type".to_string(),
              EventPayloadValue::String(Rc::new("type1".to_string())),
            ),
            (
              "object_id".to_string(),
              EventPayloadValue::String(Rc::new("id_2".to_string())),
            ),
            (
              "ocel_related_objects_ids".to_string(),
              EventPayloadValue::String(Rc::new("1 2 3 4 5".to_string())),
            ),
            (
              "ocel_related_objects_types".to_string(),
              EventPayloadValue::String(Rc::new("T1 T2 T3 T4 T5".to_string())),
            ),
          ],
        ),
        create_event_with_attributes(
          "ocel_allocate_merge".to_string(),
          vec![
            (
              "object_type".to_string(),
              EventPayloadValue::String(Rc::new("type1".to_string())),
            ),
            (
              "object_id".to_string(),
              EventPayloadValue::String(Rc::new("id_2".to_string())),
            ),
            (
              "ocel_action".to_string(),
              EventPayloadValue::String(Rc::new("AllocateMerged".to_string())),
            ),
            (
              "ocel_related_objects_ids".to_string(),
              EventPayloadValue::String(Rc::new("1 2 3 4 5".to_string())),
            ),
          ],
        ),
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      let object_id_attr = "object_id";
      let object_type_attr = SingleAttribute::new("object_type".to_string(), heaped("???".to_string()));
      let object_type_attr = NameCreationStrategy::SingleAttribute(object_type_attr);
      let base_conf = OcelObjectExtractionConfigBase::new(object_type_attr.to_owned(), object_id_attr.to_string());
      let related_ids_attr = "ocel_related_objects_ids";
      let related_types_attr = "ocel_related_objects_types";

      config.set_ocel(Some(OcelUnitedExtractionConfig::new(
        Some(" ".to_string()),
        Some(ExtractionConfig::new("ocel_allocate".to_string(), base_conf.to_owned())),
        Some(ExtractionConfig::new("ocel_consume".to_string(), base_conf.to_owned())),
        Some(ExtractionConfig::new(
          "ocel_allocate_merge".to_string(),
          OcelAllocateMergeExtractionConfig::new(base_conf.to_owned(), related_ids_attr.to_string()),
        )),
        Some(ExtractionConfig::new(
          "ocel_consume_produce".to_string(),
          OcelConsumeProduceExtractionConfig::new(
            object_id_attr.to_string(),
            related_ids_attr.to_string(),
            related_types_attr.to_string(),
          ),
        )),
      )));

      let extractor = OcelDataExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data
    },
  )
}

fn create_event_with_attributes(name: String, attributes: Vec<(String, EventPayloadValue)>) -> Rc<RefCell<XesEventImpl>> {
  Rc::new(RefCell::new(XesEventImpl::new_all_fields(
    Rc::new(name),
    Utc::now(),
    Some(attributes.into_iter().collect()),
  )))
}

fn execute_test_with_software_data(gold: &str, test: impl Fn() -> SoftwareData) {
  assert_eq!(gold, serde_json::to_string(&test()).unwrap());
}
