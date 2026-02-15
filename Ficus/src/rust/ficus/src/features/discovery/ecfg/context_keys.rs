use crate::{
  context_key,
  event_log::xes::xes_event_log::XesEventLogImpl,
  features::{
    analysis::patterns::pattern_info::{UnderlyingPatternGraphInfo, UnderlyingPatternInfo},
    discovery::{
      ecfg::models::{ActivityStartEndTimeData, CorrespondingTraceData, EdgeTraceExecutionInfo, NodeAdditionalDataContainer},
      timeline::software_data::models::SoftwareData,
    },
  },
  utils::graph::graph::DefaultGraph,
};
use lazy_static::lazy_static;

pub const NODE_SOFTWARE_DATA: &'static str = "node_software_data";
pub const NODE_CORRESPONDING_TRACE_DATA: &'static str = "node_corresponding_trace_data";
pub const NODE_INNER_GRAPH: &'static str = "node_inner_graph";
pub const NODE_START_END_ACTIVITY_TIME: &'static str = "node_start_end_activity_time";
pub const NODE_START_END_ACTIVITIES_TIMES: &'static str = "node_start_end_activities_times";
pub const NODE_UNDERLYING_PATTERNS_INFOS: &'static str = "node_underlying_patterns_infos";
pub const NODE_UNDERLYING_PATTERNS_GRAPHS_INFO: &'static str = "node_underlying_patterns_graphs_infos";
pub const NODE_MULTITHREADED_FRAGMENT_LOG: &'static str = "node_multithreaded_fragment_log";

context_key! { NODE_SOFTWARE_DATA, Vec<NodeAdditionalDataContainer<SoftwareData>> }
context_key! { NODE_CORRESPONDING_TRACE_DATA, Vec<NodeAdditionalDataContainer<CorrespondingTraceData>> }
context_key! { NODE_INNER_GRAPH, DefaultGraph }
context_key! { NODE_START_END_ACTIVITY_TIME, NodeAdditionalDataContainer<ActivityStartEndTimeData> }
context_key! { NODE_START_END_ACTIVITIES_TIMES, Vec<NodeAdditionalDataContainer<ActivityStartEndTimeData>> }
context_key! { NODE_UNDERLYING_PATTERNS_INFOS, Vec<NodeAdditionalDataContainer<UnderlyingPatternInfo>> }
context_key! { NODE_UNDERLYING_PATTERNS_GRAPHS_INFO, Vec<NodeAdditionalDataContainer<UnderlyingPatternGraphInfo>> }
context_key! { NODE_MULTITHREADED_FRAGMENT_LOG, Vec<NodeAdditionalDataContainer<XesEventLogImpl>> }

pub const EDGE_SOFTWARE_DATA: &'static str = "edge_software_data";
pub const EDGE_START_END_ACTIVITIES_TIMES: &'static str = "edge_start_end_activities_times";
pub const EDGE_TRACE_EXECUTION_INFO: &'static str = "edge_trace_execution_info";

context_key! { EDGE_SOFTWARE_DATA, Vec<SoftwareData> }
context_key! { EDGE_START_END_ACTIVITIES_TIMES, Vec<ActivityStartEndTimeData> }
context_key! { EDGE_TRACE_EXECUTION_INFO, Vec<EdgeTraceExecutionInfo> }
