use crate::features::analysis::patterns::pattern_info::{UnderlyingPatternGraphInfo, UnderlyingPatternInfo};
use crate::features::discovery::root_sequence::models::{ActivityStartEndTimeData, CorrespondingTraceData, EdgeTraceExecutionInfo, NodeAdditionalDataContainer};
use crate::features::discovery::timeline::software_data::models::SoftwareData;
use crate::utils::context_key::DefaultContextKey;
use crate::utils::graph::graph::DefaultGraph;
use lazy_static::lazy_static;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;

pub const NODE_SOFTWARE_DATA: &'static str = "node_software_data";
pub const NODE_CORRESPONDING_TRACE_DATA: &'static str = "node_corresponding_trace_data";
pub const NODE_INNER_GRAPH: &'static str = "node_inner_graph";
pub const NODE_START_END_ACTIVITY_TIME: &'static str = "node_start_end_activity_time";
pub const NODE_START_END_ACTIVITIES_TIMES: &'static str = "node_start_end_activities_times";
pub const NODE_UNDERLYING_PATTERNS_INFOS: &'static str = "node_underlying_patterns_infos";
pub const NODE_UNDERLYING_PATTERNS_GRAPHS_INFO: &'static str = "node_underlying_patterns_graphs_infos";
pub const NODE_MULTITHREADED_FRAGMENT_LOG: &'static str = "node_multithreaded_fragment_log";

lazy_static!(
  pub static ref NODE_SOFTWARE_DATA_KEY: DefaultContextKey<Vec<NodeAdditionalDataContainer<SoftwareData>>> = DefaultContextKey::new(NODE_SOFTWARE_DATA);
  pub static ref NODE_CORRESPONDING_TRACE_DATA_KEY: DefaultContextKey<Vec<NodeAdditionalDataContainer<CorrespondingTraceData>>> = DefaultContextKey::new(NODE_CORRESPONDING_TRACE_DATA);
  pub static ref NODE_INNER_GRAPH_KEY: DefaultContextKey<DefaultGraph> = DefaultContextKey::new(NODE_SOFTWARE_DATA);
  pub static ref NODE_START_END_ACTIVITY_TIME_KEY: DefaultContextKey<NodeAdditionalDataContainer<ActivityStartEndTimeData>> = DefaultContextKey::new(NODE_START_END_ACTIVITY_TIME);
  pub static ref NODE_START_END_ACTIVITIES_TIMES_KEY: DefaultContextKey<Vec<NodeAdditionalDataContainer<ActivityStartEndTimeData>>> = DefaultContextKey::new(NODE_START_END_ACTIVITIES_TIMES);
  pub static ref NODE_UNDERLYING_PATTERNS_INFOS_KEY: DefaultContextKey<Vec<NodeAdditionalDataContainer<UnderlyingPatternInfo>>> = DefaultContextKey::new(NODE_UNDERLYING_PATTERNS_INFOS);
  pub static ref NODE_UNDERLYING_PATTERNS_GRAPHS_INFOS_KEY: DefaultContextKey<Vec<NodeAdditionalDataContainer<UnderlyingPatternGraphInfo>>> = DefaultContextKey::new(NODE_UNDERLYING_PATTERNS_GRAPHS_INFO);
  pub static ref NODE_MULTITHREADED_FRAGMENT_LOG_KEY: DefaultContextKey<Vec<NodeAdditionalDataContainer<XesEventLogImpl>>> = DefaultContextKey::new(NODE_MULTITHREADED_FRAGMENT_LOG);
);

pub const EDGE_SOFTWARE_DATA: &'static str = "edge_software_data";
pub const EDGE_START_END_ACTIVITIES_TIMES: &'static str = "edge_start_end_activities_times";
pub const EDGE_TRACE_EXECUTION_INFO: &'static str = "edge_trace_execution_info";

lazy_static!(
  pub static ref EDGE_SOFTWARE_DATA_KEY: DefaultContextKey<Vec<SoftwareData>> = DefaultContextKey::new(EDGE_SOFTWARE_DATA);
  pub static ref EDGE_START_END_ACTIVITIES_TIMES_KEY: DefaultContextKey<Vec<ActivityStartEndTimeData>> = DefaultContextKey::new(EDGE_START_END_ACTIVITIES_TIMES);
  pub static ref EDGE_TRACE_EXECUTION_INFO_KEY: DefaultContextKey<Vec<EdgeTraceExecutionInfo>> = DefaultContextKey::new(EDGE_TRACE_EXECUTION_INFO);
);