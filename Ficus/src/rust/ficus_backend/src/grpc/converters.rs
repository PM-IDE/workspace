use std::collections::HashMap;
use std::fmt::Display;
use std::{any::Any, str::FromStr};

use nameof::name_of_type;

use crate::features::analysis::patterns::activity_instances::{ActivityInTraceFilterKind, ActivityNarrowingKind};
use crate::features::clustering::activities::activities_params::ActivityRepresentationSource;
use crate::features::clustering::traces::traces_params::TracesRepresentationSource;
use crate::features::discovery::petri_net::arc::Arc;
use crate::features::discovery::petri_net::marking::{Marking, SingleMarking};
use crate::features::discovery::petri_net::petri_net::DefaultPetriNet;
use crate::features::discovery::petri_net::place::Place;
use crate::features::discovery::petri_net::transition::Transition;
use crate::ficus_proto::{
    GrpcColorsEventLogMapping, GrpcCountAnnotation, GrpcDataset, GrpcEntityCountAnnotation, GrpcEntityFrequencyAnnotation,
    GrpcFrequenciesAnnotation, GrpcGraph, GrpcGraphEdge, GrpcGraphNode, GrpcLabeledDataset, GrpcMatixRow, GrpcMatrix, GrpcPetriNet,
    GrpcPetriNetArc, GrpcPetriNetMarking, GrpcPetriNetPlace, GrpcPetriNetSinglePlaceMarking, GrpcPetriNetTransition,
};
use crate::pipelines::activities_parts::{ActivitiesLogsSourceDto, UndefActivityHandlingStrategyDto};
use crate::pipelines::patterns_parts::PatternsKindDto;
use crate::utils::colors::ColorsEventLog;
use crate::utils::dataset::dataset::{FicusDataset, LabeledDataset};
use crate::utils::distance::distance::FicusDistance;
use crate::utils::graph::graph::{DefaultGraph, Graph};
use crate::utils::graph::graph_edge::GraphEdge;
use crate::utils::graph::graph_node::GraphNode;
use crate::utils::log_serialization_format::LogSerializationFormat;
use crate::{
    features::analysis::{
        event_log_info::EventLogInfo,
        patterns::{
            activity_instances::AdjustingMode, contexts::PatternsDiscoveryStrategy, repeat_sets::SubArrayWithTraceIndex,
            tandem_arrays::SubArrayInTraceInfo,
        },
    },
    ficus_proto::{
        grpc_context_value::ContextValue, GrpcColor, GrpcColoredRectangle, GrpcColorsEventLog, GrpcColorsTrace, GrpcContextValue,
        GrpcEventLogInfo, GrpcEventLogTraceSubArraysContextValue, GrpcHashesEventLog, GrpcHashesEventLogContextValue, GrpcHashesLogTrace,
        GrpcNamesEventLog, GrpcNamesEventLogContextValue, GrpcNamesTrace, GrpcSubArrayWithTraceIndex,
        GrpcSubArraysWithTraceIndexContextValue, GrpcTraceSubArray, GrpcTraceSubArrays,
    },
    pipelines::{
        context::PipelineContext,
        keys::{context_key::ContextKey, context_keys::ContextKeys},
        pipelines::Pipeline,
    },
    utils::{
        colors::{Color, ColoredRectangle},
        user_data::{keys::Key, user_data::UserData},
    },
};

use super::backend_service::{FicusService, ServicePipelineExecutionContext};

pub(super) fn create_initial_context<'a>(context: &'a ServicePipelineExecutionContext) -> PipelineContext<'a> {
    let mut pipeline_context = PipelineContext::new_with_logging(context.parts());

    for value in context.context_values() {
        let key = context.keys().find_key(&value.key.as_ref().unwrap().name).unwrap();
        let value = value.value.as_ref().unwrap().context_value.as_ref().unwrap();
        put_into_user_data(key.key(), value, &mut pipeline_context, context);
    }

    pipeline_context
}

pub(super) fn put_into_user_data(
    key: &dyn Key,
    value: &ContextValue,
    user_data: &mut impl UserData,
    context: &ServicePipelineExecutionContext,
) {
    match value {
        ContextValue::String(string) => user_data.put_any::<String>(key, string.clone()),
        ContextValue::HashesLog(_) => todo!(),
        ContextValue::NamesLog(grpc_log) => put_names_log_to_context(key, grpc_log, user_data),
        ContextValue::Uint32(number) => user_data.put_any::<u32>(key, number.clone()),
        ContextValue::TracesSubArrays(_) => todo!(),
        ContextValue::TraceIndexSubArrays(_) => todo!(),
        ContextValue::Bool(bool) => user_data.put_any::<bool>(key, bool.clone()),
        ContextValue::XesEventLog(grpc_log) => put_names_log_to_context(key, grpc_log, user_data),
        ContextValue::ColorsLog(_) => {}
        ContextValue::Enum(grpc_enum) => {
            let enum_name = &grpc_enum.enum_type;
            if enum_name == name_of_type!(PatternsDiscoveryStrategy) {
                parse_grpc_enum::<PatternsDiscoveryStrategy>(user_data, key, &grpc_enum.value);
            } else if enum_name == name_of_type!(AdjustingMode) {
                parse_grpc_enum::<AdjustingMode>(user_data, key, &grpc_enum.value);
            } else if enum_name == name_of_type!(PatternsKindDto) {
                parse_grpc_enum::<PatternsKindDto>(user_data, key, &grpc_enum.value);
            } else if enum_name == name_of_type!(UndefActivityHandlingStrategyDto) {
                parse_grpc_enum::<UndefActivityHandlingStrategyDto>(user_data, key, &grpc_enum.value);
            } else if enum_name == name_of_type!(ActivityNarrowingKind) {
                parse_grpc_enum::<ActivityNarrowingKind>(user_data, key, &grpc_enum.value);
            } else if enum_name == name_of_type!(ActivityInTraceFilterKind) {
                parse_grpc_enum::<ActivityInTraceFilterKind>(user_data, key, &grpc_enum.value);
            } else if enum_name == name_of_type!(ActivitiesLogsSourceDto) {
                parse_grpc_enum::<ActivitiesLogsSourceDto>(user_data, key, &grpc_enum.value);
            } else if enum_name == name_of_type!(ActivityRepresentationSource) {
                parse_grpc_enum::<ActivityRepresentationSource>(user_data, key, &grpc_enum.value);
            } else if enum_name == name_of_type!(FicusDistance) {
                parse_grpc_enum::<FicusDistance>(user_data, key, &grpc_enum.value);
            } else if enum_name == name_of_type!(TracesRepresentationSource) {
                parse_grpc_enum::<TracesRepresentationSource>(user_data, key, &grpc_enum.value);
            } else if enum_name == name_of_type!(LogSerializationFormat) {
                parse_grpc_enum::<LogSerializationFormat>(user_data, key, &grpc_enum.value);
            }
        }
        ContextValue::EventLogInfo(_) => todo!(),
        ContextValue::Strings(strings) => user_data.put_any::<Vec<String>>(key, strings.strings.clone()),
        ContextValue::Pipeline(pipeline) => {
            let pipeline = FicusService::to_pipeline(&context.with_pipeline(pipeline));
            user_data.put_any::<Pipeline>(key, pipeline);
        }
        ContextValue::PetriNet(_) => todo!(),
        ContextValue::Graph(_) => todo!(),
        ContextValue::Float(value) => user_data.put_any::<f64>(key, *value as f64),
        ContextValue::CountAnnotation(_) => todo!(),
        ContextValue::FrequencyAnnotation(_) => todo!(),
        ContextValue::Dataset(_) => todo!(),
        ContextValue::LabeledDataset(_) => todo!(),
        ContextValue::Bytes(grpc_bytes) => user_data.put_any::<Vec<u8>>(key, grpc_bytes.bytes.clone()),
    }
}

fn parse_grpc_enum<TEnum: FromStr + 'static>(user_data: &mut impl UserData, key: &dyn Key, raw_enum: &str) {
    if let Ok(parsed_value) = TEnum::from_str(raw_enum) {
        user_data.put_any::<TEnum>(key, parsed_value);
    }
}

fn put_names_log_to_context(key: &dyn Key, grpc_log: &GrpcNamesEventLogContextValue, user_data: &mut impl UserData) {
    let grpc_log = grpc_log.log.as_ref().unwrap();
    let mut names_log = vec![];
    for grpc_trace in &grpc_log.traces {
        let mut trace = vec![];
        for grpc_event in &grpc_trace.events {
            trace.push(grpc_event.clone());
        }

        names_log.push(trace);
    }

    user_data.put_any::<Vec<Vec<String>>>(key, names_log);
}

pub fn convert_to_grpc_context_value(key: &dyn ContextKey, value: &dyn Any, keys: &ContextKeys) -> Option<GrpcContextValue> {
    if keys.is_path(key) {
        try_convert_to_string_context_value(value)
    } else if keys.is_hashes_event_log(key) {
        try_convert_to_hashes_event_log(value)
    } else if keys.is_names_event_log(key) {
        try_convert_to_names_event_log(value)
    } else if keys.is_patterns(key) {
        try_convert_to_grpc_traces_sub_arrays(value)
    } else if keys.is_repeat_sets(key) {
        try_convert_to_grpc_sub_arrays_with_index(value)
    } else if keys.is_colors_event_log(key) {
        try_convert_to_grpc_colors_event_log(value)
    } else if keys.is_event_log_info(key) {
        try_convert_to_grpc_event_log_info(value)
    } else if keys.is_petri_net(key) {
        try_convert_to_grpc_petri_net(value)
    } else if keys.is_graph(key) {
        try_convert_to_grpc_graph(value)
    } else if keys.is_petri_net_count_annotation(key) {
        try_convert_to_grpc_petri_net_count_annotation(value)
    } else if keys.is_petri_net_frequency_annotation(key) {
        try_convert_to_grpc_petri_net_frequency_annotation(value)
    } else if keys.is_petri_net_trace_frequency_annotation(key) {
        try_convert_to_grpc_petri_net_frequency_annotation(value)
    } else if keys.is_traces_activities_dataset(key) {
        try_convert_to_grpc_dataset(value)
    } else if keys.is_labeled_traces_activities_dataset(key) {
        try_convert_to_grpc_labeled_dataset(value)
    } else if keys.is_labeled_log_traces_dataset(key) {
        try_convert_to_grpc_labeled_dataset(value)
    } else if keys.is_log_traces_dataset(key) {
        try_convert_to_grpc_dataset(value)
    } else {
        None
    }
}

fn try_convert_to_grpc_petri_net_count_annotation(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<HashMap<u64, usize>>() {
        None
    } else {
        let value = value.downcast_ref::<HashMap<u64, usize>>().unwrap();
        Some(GrpcContextValue {
            context_value: Some(ContextValue::CountAnnotation(convert_to_grpc_count_annotation(value))),
        })
    }
}

fn try_convert_to_grpc_labeled_dataset(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<LabeledDataset>() {
        None
    } else {
        let value = value.downcast_ref::<LabeledDataset>().unwrap();
        Some(GrpcContextValue {
            context_value: Some(ContextValue::LabeledDataset(convert_to_labeled_grpc_dataset(value))),
        })
    }
}

fn try_convert_to_grpc_petri_net_frequency_annotation(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<HashMap<u64, f64>>() {
        None
    } else {
        let value = value.downcast_ref::<HashMap<u64, f64>>().unwrap();
        Some(GrpcContextValue {
            context_value: Some(ContextValue::FrequencyAnnotation(convert_to_grpc_frequency_annotation(value))),
        })
    }
}

fn try_convert_to_string_context_value(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<String>() {
        None
    } else {
        Some(GrpcContextValue {
            context_value: Some(ContextValue::String(value.downcast_ref::<String>().unwrap().clone())),
        })
    }
}

fn try_convert_to_hashes_event_log(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<Vec<Vec<u64>>>() {
        None
    } else {
        let vec = value.downcast_ref::<Vec<Vec<u64>>>().unwrap();
        let mut traces = vec![];
        for trace in vec {
            let mut events = vec![];
            for event in trace {
                events.push(*event);
            }

            traces.push(GrpcHashesLogTrace { events });
        }

        Some(GrpcContextValue {
            context_value: Some(ContextValue::HashesLog(GrpcHashesEventLogContextValue {
                log: Some(GrpcHashesEventLog { traces }),
            })),
        })
    }
}

fn try_convert_to_names_event_log(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<Vec<Vec<String>>>() {
        None
    } else {
        let vec = value.downcast_ref::<Vec<Vec<String>>>().unwrap();
        let mut traces = vec![];
        for trace in vec {
            let mut events = vec![];
            for event in trace {
                events.push(event.clone());
            }

            traces.push(GrpcNamesTrace { events });
        }

        Some(GrpcContextValue {
            context_value: Some(ContextValue::NamesLog(GrpcNamesEventLogContextValue {
                log: Some(GrpcNamesEventLog { traces }),
            })),
        })
    }
}

fn try_convert_to_grpc_traces_sub_arrays(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<Vec<Vec<SubArrayInTraceInfo>>>() {
        None
    } else {
        let vec = value.downcast_ref::<Vec<Vec<SubArrayInTraceInfo>>>().unwrap();
        let mut traces = vec![];
        for trace in vec {
            let mut sub_arrays = vec![];
            for array in trace {
                sub_arrays.push(GrpcTraceSubArray {
                    start: array.start_index as u32,
                    end: (array.start_index + array.length) as u32,
                })
            }

            traces.push(GrpcTraceSubArrays { sub_arrays })
        }

        Some(GrpcContextValue {
            context_value: Some(ContextValue::TracesSubArrays(GrpcEventLogTraceSubArraysContextValue {
                traces_sub_arrays: traces,
            })),
        })
    }
}

fn try_convert_to_grpc_sub_arrays_with_index(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<Vec<SubArrayWithTraceIndex>>() {
        None
    } else {
        let vec = value.downcast_ref::<Vec<SubArrayWithTraceIndex>>().unwrap();
        let mut sub_arrays = vec![];

        for array in vec {
            sub_arrays.push(GrpcSubArrayWithTraceIndex {
                sub_array: Some(GrpcTraceSubArray {
                    start: array.sub_array.start_index as u32,
                    end: (array.sub_array.start_index + array.sub_array.length) as u32,
                }),
                trace_index: array.trace_index as u32,
            })
        }

        Some(GrpcContextValue {
            context_value: Some(ContextValue::TraceIndexSubArrays(GrpcSubArraysWithTraceIndexContextValue {
                sub_arrays,
            })),
        })
    }
}

fn try_convert_to_grpc_colors_event_log(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<ColorsEventLog>() {
        None
    } else {
        let colors_log = value.downcast_ref::<ColorsEventLog>().unwrap();
        let mut grpc_traces = vec![];
        let mut mapping = HashMap::new();
        let mut grpc_mapping = vec![];
        for (key, color) in colors_log.mapping.iter() {
            mapping.insert(key.to_owned(), grpc_mapping.len());
            grpc_mapping.push(GrpcColorsEventLogMapping {
                name: key.to_owned(),
                color: Some(convert_to_grpc_color(color)),
            });
        }

        for trace in &colors_log.traces {
            let mut grpc_trace = vec![];
            let mut last_width = None;
            let mut same_width = true;

            for colored_rect in trace {
                if same_width {
                    if let Some(last_width) = last_width {
                        if last_width != colored_rect.len() {
                            same_width = false;
                        }
                    }

                    last_width = Some(colored_rect.len())
                }

                let index = *mapping.get(colored_rect.name()).unwrap();
                grpc_trace.push(convert_to_grpc_colored_rect(colored_rect, index))
            }

            grpc_traces.push(GrpcColorsTrace {
                event_colors: grpc_trace,
                constant_width: same_width,
            })
        }

        Some(GrpcContextValue {
            context_value: Some(ContextValue::ColorsLog(GrpcColorsEventLog {
                mapping: grpc_mapping,
                traces: grpc_traces,
            })),
        })
    }
}

fn convert_to_grpc_colored_rect(colored_rect: &ColoredRectangle, color_index: usize) -> GrpcColoredRectangle {
    GrpcColoredRectangle {
        color_index: color_index as u32,
        start_index: colored_rect.start_pos() as u32,
        length: colored_rect.len() as u32,
    }
}

fn convert_to_grpc_color(color: &Color) -> GrpcColor {
    GrpcColor {
        red: color.red() as u32,
        green: color.green() as u32,
        blue: color.blue() as u32,
    }
}

fn try_convert_to_grpc_event_log_info(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<EventLogInfo>() {
        None
    } else {
        let log_info = value.downcast_ref::<EventLogInfo>().unwrap();
        Some(GrpcContextValue {
            context_value: Some(ContextValue::EventLogInfo(GrpcEventLogInfo {
                events_count: log_info.events_count() as u32,
                traces_count: log_info.traces_count() as u32,
                event_classes_count: log_info.event_classes_count() as u32,
            })),
        })
    }
}

fn try_convert_to_grpc_petri_net(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<DefaultPetriNet>() {
        None
    } else {
        let petri_net = value.downcast_ref::<DefaultPetriNet>().unwrap();
        let grpc_places: Vec<GrpcPetriNetPlace> = petri_net.all_places().iter().map(|place| convert_to_grpc_place(place)).collect();

        let grpc_transitions: Vec<GrpcPetriNetTransition> = petri_net
            .all_transitions()
            .iter()
            .map(|transition| convert_to_grpc_transition(transition))
            .collect();

        Some(GrpcContextValue {
            context_value: Some(ContextValue::PetriNet(GrpcPetriNet {
                places: grpc_places,
                transitions: grpc_transitions,
                initial_marking: try_convert_to_grpc_marking(petri_net.initial_marking()),
                final_marking: try_convert_to_grpc_marking(petri_net.final_marking()),
            })),
        })
    }
}

fn convert_to_grpc_place(place: &Place) -> GrpcPetriNetPlace {
    GrpcPetriNetPlace {
        id: place.id() as i64,
        name: place.name().to_owned(),
    }
}

fn convert_to_grpc_transition<TTransitionData, TArcData>(transition: &Transition<TTransitionData, TArcData>) -> GrpcPetriNetTransition
where
    TTransitionData: ToString,
{
    let incoming_arcs = transition
        .incoming_arcs()
        .iter()
        .map(|arc| convert_to_grpc_arc(arc))
        .collect::<Vec<GrpcPetriNetArc>>();

    let outgoing_arcs = transition
        .outgoing_arcs()
        .iter()
        .map(|arc| convert_to_grpc_arc(arc))
        .collect::<Vec<GrpcPetriNetArc>>();

    GrpcPetriNetTransition {
        id: transition.id() as i64,
        incoming_arcs,
        outgoing_arcs,
        data: match transition.data() {
            None => "".to_string(),
            Some(data) => data.to_string(),
        },
    }
}

fn convert_to_grpc_arc<TArcData>(arc: &Arc<TArcData>) -> GrpcPetriNetArc {
    GrpcPetriNetArc {
        id: arc.id() as i64,
        place_id: arc.place_id() as i64,
        tokens_count: *arc.tokens_count() as i64,
    }
}

fn try_convert_to_grpc_marking(marking: Option<&Marking>) -> Option<GrpcPetriNetMarking> {
    match marking {
        None => None,
        Some(marking) => Some(GrpcPetriNetMarking {
            markings: marking
                .active_places()
                .iter()
                .map(|single_marking| convert_to_grpc_single_marking(single_marking))
                .collect(),
        }),
    }
}

fn convert_to_grpc_single_marking(marking: &SingleMarking) -> GrpcPetriNetSinglePlaceMarking {
    GrpcPetriNetSinglePlaceMarking {
        place_id: marking.place_id() as i64,
        tokens_count: marking.tokens_count() as i64,
    }
}

fn try_convert_to_grpc_graph(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<DefaultGraph>() {
        None
    } else {
        let graph = value.downcast_ref::<DefaultGraph>().unwrap();
        Some(GrpcContextValue {
            context_value: Some(ContextValue::Graph(convert_to_grpc_graph(graph))),
        })
    }
}

fn convert_to_grpc_graph<TNodeData, TEdgeData>(graph: &Graph<TNodeData, TEdgeData>) -> GrpcGraph
where
    TNodeData: ToString,
    TEdgeData: ToString + Display,
{
    let nodes: Vec<GrpcGraphNode> = graph.all_nodes().iter().map(|node| convert_to_grpc_graph_node(*node)).collect();
    let edges: Vec<GrpcGraphEdge> = graph.all_edges().iter().map(|edge| convert_to_grpc_graph_edge(edge)).collect();

    GrpcGraph { edges, nodes }
}

fn convert_to_grpc_graph_node<TNodeData>(node: &GraphNode<TNodeData>) -> GrpcGraphNode
where
    TNodeData: ToString,
{
    GrpcGraphNode {
        id: *node.id(),
        data: match node.data() {
            None => "".to_string(),
            Some(data) => data.to_string(),
        },
    }
}

fn convert_to_grpc_graph_edge<TEdgeData>(edge: &GraphEdge<TEdgeData>) -> GrpcGraphEdge
where
    TEdgeData: ToString,
{
    GrpcGraphEdge {
        from_node: *edge.from_node(),
        to_node: *edge.to_node(),
        data: match edge.data() {
            None => "".to_string(),
            Some(data) => data.to_string(),
        },
    }
}

fn convert_to_grpc_count_annotation(annotation: &HashMap<u64, usize>) -> GrpcCountAnnotation {
    let annotations = annotation
        .iter()
        .map(|pair| GrpcEntityCountAnnotation {
            entity_id: *pair.0 as i64,
            count: *pair.1 as i64,
        })
        .collect();

    GrpcCountAnnotation { annotations }
}

fn convert_to_grpc_frequency_annotation(annotation: &HashMap<u64, f64>) -> GrpcFrequenciesAnnotation {
    let annotations = annotation
        .iter()
        .map(|pair| GrpcEntityFrequencyAnnotation {
            entity_id: *pair.0 as i64,
            frequency: *pair.1 as f32,
        })
        .collect();

    GrpcFrequenciesAnnotation { annotations }
}

fn try_convert_to_grpc_dataset(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<FicusDataset>() {
        None
    } else {
        Some(GrpcContextValue {
            context_value: Some(ContextValue::Dataset(convert_to_grpc_dataset(
                value.downcast_ref::<FicusDataset>().unwrap(),
            ))),
        })
    }
}

fn convert_to_grpc_dataset(dataset: &FicusDataset) -> GrpcDataset {
    let rows = dataset
        .values()
        .iter()
        .map(|x| GrpcMatixRow {
            values: x.iter().map(|x| *x as f32).collect(),
        })
        .collect();

    let matrix = GrpcMatrix { rows };

    GrpcDataset {
        matrix: Some(matrix),
        columns_names: dataset.columns_names().clone(),
        row_names: dataset.row_names().clone(),
    }
}

fn convert_to_labeled_grpc_dataset(dataset: &LabeledDataset) -> GrpcLabeledDataset {
    let grpc_dataset = convert_to_grpc_dataset(dataset.dataset());
    let labels = dataset.labels().iter().map(|x| *x as i32).collect();
    let labels_colors = dataset.colors().iter().map(|x| convert_to_grpc_color(x)).collect();

    GrpcLabeledDataset {
        dataset: Some(grpc_dataset),
        labels,
        labels_colors,
    }
}
