use crate::test_core::simple_events_logs_provider::{create_event_log_with_simple_real_time, create_simple_event_log};
use ficus::event_log::xes::xes_event_log::XesEventLogImpl;
use ficus::features::analysis::directly_follows_graph::construct_dfg;
use ficus::features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto};
use ficus::features::discovery::alpha::alpha::discover_petri_net_alpha;
use ficus::features::discovery::alpha::providers::alpha_provider::DefaultAlphaRelationsProvider;
use ficus::features::discovery::petri_net::annotations::{
    annotate_with_counts, annotate_with_frequencies, annotate_with_time_performance, annotate_with_trace_frequency, TimeAnnotationKind,
};
use ficus::features::discovery::petri_net::petri_net::DefaultPetriNet;
use ficus::utils::graph::graph::DefaultGraph;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;

#[test]
pub fn test_simple_count_annotation() {
    let log = create_simple_event_log();
    let log_info = EventLogInfo::create_from(EventLogInfoCreationDto::default(&log));
    let petri_net = discover_petri_net_alpha(&DefaultAlphaRelationsProvider::new(&log_info));
    let annotation = annotate_with_counts(&log, &petri_net, true).unwrap();

    execute_test_with_annotation(
        &petri_net,
        annotation,
        vec![
            ("({A}, {B})--A".to_owned(), 2),
            ("({A}, {B})--B".to_owned(), 2),
            ("({B}, {C})--B".to_owned(), 2),
            ("({B}, {C})--C".to_owned(), 2),
            ("EndPlace--C".to_owned(), 2),
            ("StartPlace--A".to_owned(), 2),
        ],
    );
}

pub fn execute_test_with_annotation<T>(net: &DefaultPetriNet, annotation: HashMap<u64, T>, mut expected: Vec<(String, T)>)
where
    T: ToString + PartialEq + Debug + Copy,
{
    let mut processed_annotations: Vec<(String, T)> = annotation
        .iter()
        .map(|pair| {
            if let Some((arc, transition)) = net.arc(pair.0) {
                let place = net.place(&arc.place_id());
                let name = format!("{}--{}", place.name(), transition.name());
                return (name, *pair.1);
            }

            panic!();
        })
        .collect();

    processed_annotations.sort_by(|first, second| first.0.cmp(&second.0));
    expected.sort_by(|first, second| first.0.cmp(&second.0));

    assert_eq!(processed_annotations, expected);
}

#[test]
pub fn test_simple_frequency_annotation() {
    let log = create_simple_event_log();
    let log_info = EventLogInfo::create_from(EventLogInfoCreationDto::default(&log));
    let petri_net = discover_petri_net_alpha(&DefaultAlphaRelationsProvider::new(&log_info));
    let annotation = annotate_with_frequencies(&log, &petri_net, true).unwrap();

    execute_test_with_annotation(
        &petri_net,
        annotation,
        vec![
            ("({A}, {B})--A".to_owned(), 0.16666666666666666),
            ("({A}, {B})--B".to_owned(), 0.16666666666666666),
            ("({B}, {C})--B".to_owned(), 0.16666666666666666),
            ("({B}, {C})--C".to_owned(), 0.16666666666666666),
            ("EndPlace--C".to_owned(), 0.16666666666666666),
            ("StartPlace--A".to_owned(), 0.16666666666666666),
        ],
    );
}

#[test]
pub fn test_simple_trace_frequency_annotation() {
    let log = create_simple_event_log();
    let log_info = EventLogInfo::create_from(EventLogInfoCreationDto::default(&log));
    let petri_net = discover_petri_net_alpha(&DefaultAlphaRelationsProvider::new(&log_info));
    let annotation = annotate_with_trace_frequency(&log, &petri_net, true).unwrap();

    execute_test_with_annotation(
        &petri_net,
        annotation,
        vec![
            ("({A}, {B})--A".to_owned(), 1.0),
            ("({A}, {B})--B".to_owned(), 1.0),
            ("({B}, {C})--B".to_owned(), 1.0),
            ("({B}, {C})--C".to_owned(), 1.0),
            ("EndPlace--C".to_owned(), 1.0),
            ("StartPlace--A".to_owned(), 1.0),
        ],
    );
}

#[test]
pub fn test_simple_time_annotation_summed() {
    execute_time_annotation_test(
        create_event_log_with_simple_real_time(),
        TimeAnnotationKind::SummedTime,
        vec![
            ("A---B".to_string(), 2.0),
            ("A---A".to_string(), 4.0),
            ("B---C".to_string(), 4.0),
            ("E---C".to_string(), 1.0),
            ("C---B".to_string(), 1.0),
            ("C---D".to_string(), 1.0),
            ("D---E".to_string(), 1.0),
            ("C---C".to_string(), 1.0),
            ("C---A".to_string(), 1.0),
            ("C---E".to_string(), 2.0),
            ("B---E".to_string(), 1.0),
            ("B---B".to_string(), 2.0),
            ("E---A".to_string(), 3.0),
            ("A---E".to_string(), 1.0),
        ],
    );
}

#[test]
pub fn test_simple_time_annotation_mean() {
    execute_time_annotation_test(
        create_event_log_with_simple_real_time(),
        TimeAnnotationKind::Mean,
        vec![
            ("B---B".to_string(), 1.0),
            ("A---A".to_string(), 1.0),
            ("E---C".to_string(), 1.0),
            ("D---E".to_string(), 1.0),
            ("A---B".to_string(), 1.0),
            ("C---C".to_string(), 1.0),
            ("C---A".to_string(), 1.0),
            ("C---D".to_string(), 1.0),
            ("C---B".to_string(), 1.0),
            ("C---E".to_string(), 1.0),
            ("B---E".to_string(), 1.0),
            ("E---A".to_string(), 1.0),
            ("A---E".to_string(), 1.0),
            ("B---C".to_string(), 1.0),
        ],
    );
}

fn execute_time_annotation_test(log: XesEventLogImpl, annotation_kind: TimeAnnotationKind, mut gold: Vec<(String, f64)>) {
    let info = EventLogInfo::create_from(EventLogInfoCreationDto::default(&log));

    let graph = construct_dfg(&info);

    let annotation = annotate_with_time_performance(&log, &graph, annotation_kind);
    let mut annotation = create_time_annotation_gold(annotation.as_ref().unwrap(), &graph);

    annotation.sort_by(|f, s| f.0.cmp(&s.0));
    gold.sort_by(|f, s| f.0.cmp(&s.0));

    assert_eq!(annotation, gold);
}

fn create_time_annotation_gold(annotation: &HashMap<u64, f64>, graph: &DefaultGraph) -> Vec<(String, f64)> {
    annotation
        .iter()
        .map(|p| {
            for edge in graph.all_edges() {
                if edge.id() == p.0 {
                    let first_name = graph.node(edge.from_node()).unwrap().data().unwrap().deref().to_owned();
                    let second_name = graph.node(edge.to_node()).unwrap().data().unwrap().deref().to_owned();

                    return (first_name + "---" + second_name.as_str(), *p.1);
                }
            }

            panic!("Edge should be found");
        })
        .collect::<Vec<(String, f64)>>()
}
