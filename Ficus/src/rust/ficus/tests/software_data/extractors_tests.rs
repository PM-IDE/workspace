use chrono::Utc;
use ficus::event_log::core::event::event::EventPayloadValue;
use ficus::event_log::xes::xes_event::XesEventImpl;
use ficus::features::discovery::timeline::events_groups::EventGroup;
use ficus::features::discovery::timeline::software_data::extraction_config::{
  ActivityDurationExtractionConfig, ExtractionConfig, GenericExtractionConfigBase, NameCreationStrategy, PieChartExtractionConfig,
  SimpleCountExtractionConfig, SingleAttribute, SoftwareDataExtractionConfig, TimeAttributeConfig, TimeKind,
};
use ficus::features::discovery::timeline::software_data::extractors::activities_durations::ActivityDurationExtractor;
use ficus::features::discovery::timeline::software_data::extractors::core::{
  EventGroupSoftwareDataExtractor, EventGroupTraceSoftwareDataExtractor,
};
use ficus::features::discovery::timeline::software_data::extractors::pie_charts::PieChartExtractor;
use ficus::features::discovery::timeline::software_data::extractors::simple_counter::SimpleCounterExtractor;
use ficus::features::discovery::timeline::software_data::models::SoftwareData;
use std::cell::RefCell;
use std::rc::Rc;

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
              EventPayloadValue::String(Rc::new(Box::new("type1".to_string()))),
            ),
            ("count".to_string(), EventPayloadValue::Float64(123.)),
          ],
        ),
        create_event_with_attributes(
          "histogram_event".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new(Box::new("type2".to_string()))),
            ),
            ("count".to_string(), EventPayloadValue::Float32(123.)),
          ],
        ),
        create_event_with_attributes(
          "histogram_event".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new(Box::new("type1".to_string()))),
            ),
            ("count".to_string(), EventPayloadValue::Uint64(123)),
          ],
        ),
        create_event_with_attributes(
          "unknown".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new(Box::new("type1".to_string()))),
            ),
            ("count".to_string(), EventPayloadValue::Uint32(123)),
          ],
        ),
        create_event_with_attributes(
          "hst_event".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new(Box::new("type1".to_string()))),
            ),
            ("count".to_string(), EventPayloadValue::Int64(123)),
          ],
        ),
        create_event_with_attributes(
          "hst_event".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new(Box::new("type2".to_string()))),
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
            GenericExtractionConfigBase::new("g1".to_string(), "units".to_string(), None),
            Some(NameCreationStrategy::SingleAttribute(SingleAttribute::new(
              "type".to_string(),
              "xd".to_string(),
            ))),
            Some("count".to_string()),
          ),
        ),
        ExtractionConfig::new(
          "hst_event".to_string(),
          PieChartExtractionConfig::new(
            GenericExtractionConfigBase::new("g2".to_string(), "units".to_string(), None),
            Some(NameCreationStrategy::SingleAttribute(SingleAttribute::new(
              "type".to_string(),
              "xd".to_string(),
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
              EventPayloadValue::String(Rc::new(Box::new("type1".to_string()))),
            ),
            ("count".to_string(), EventPayloadValue::Float64(123.)),
          ],
        ),
        create_event_with_attributes(
          "histogram_event".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new(Box::new("type2".to_string()))),
            ),
            ("count".to_string(), EventPayloadValue::Float32(123.)),
          ],
        ),
        create_event_with_attributes(
          "histogram_event".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new(Box::new("type1".to_string()))),
            ),
            ("count".to_string(), EventPayloadValue::Uint64(123)),
          ],
        ),
        create_event_with_attributes(
          "unknown".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new(Box::new("type1".to_string()))),
            ),
            ("count".to_string(), EventPayloadValue::Uint32(123)),
          ],
        ),
        create_event_with_attributes(
          "hst_event".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new(Box::new("type1".to_string()))),
            ),
            ("count".to_string(), EventPayloadValue::Int64(123)),
          ],
        ),
        create_event_with_attributes(
          "hst_event".to_string(),
          vec![
            (
              "type".to_string(),
              EventPayloadValue::String(Rc::new(Box::new("type2".to_string()))),
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
            GenericExtractionConfigBase::new("counter1".to_string(), "units".to_string(), None),
            None,
          ),
        ),
        ExtractionConfig::new(
          "hst_event".to_string(),
          SimpleCountExtractionConfig::new(
            GenericExtractionConfigBase::new("counter2".to_string(), "units".to_string(), None),
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
              EventPayloadValue::String(Rc::new(Box::new("1".to_string()))),
            ),
            ("stamp".to_string(), EventPayloadValue::Int64(100)),
          ],
        ),
        create_event_with_attributes(
          "event_end".to_string(),
          vec![
            (
              "activity_id".to_string(),
              EventPayloadValue::String(Rc::new(Box::new("1".to_string()))),
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
            EventPayloadValue::String(Rc::new(Box::new("2".to_string()))),
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
    GenericExtractionConfigBase::new("activity".to_string(), "units".to_string(), None),
    "event_start".to_string(),
    "event_end".to_string(),
    Some(TimeAttributeConfig::new("stamp".to_string(), TimeKind::Unknown)),
    Some(NameCreationStrategy::SingleAttribute(SingleAttribute::new(
      "activity_id".to_string(),
      "xd".to_string(),
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
              EventPayloadValue::String(Rc::new(Box::new("2".to_string()))),
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

fn create_event_with_attributes(name: String, attributes: Vec<(String, EventPayloadValue)>) -> Rc<RefCell<XesEventImpl>> {
  Rc::new(RefCell::new(XesEventImpl::new_all_fields(
    Rc::new(Box::new(name)),
    Utc::now(),
    Some(attributes.into_iter().collect()),
  )))
}

fn execute_test_with_software_data(gold: &str, test: impl Fn() -> SoftwareData) {
  assert_eq!(gold, serde_json::to_string(&test()).unwrap());
}
