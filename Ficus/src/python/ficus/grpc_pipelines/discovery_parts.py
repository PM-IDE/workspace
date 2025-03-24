from .entry_points.default_pipeline import *
from .models.pipelines_and_context_pb2 import *
from .data_models import RootSequenceKind
from ..legacy.discovery.graph import draw_graph
from ..legacy.discovery.petri_net import draw_petri_net


class DiscoverPetriNetAlpha(PipelinePart):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    return _create_default_discovery_part(const_discover_petri_net_alpha)


class DiscoverPetriNetAlphaStream(PipelinePart):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    return _create_default_discovery_part(const_discover_petri_net_alpha_stream)


def _create_default_discovery_part(algo_name: str) -> GrpcPipelinePartBase:
  config = GrpcPipelinePartConfiguration()
  return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(algo_name, config))


class DiscoverPetriNetAlphaPlus(PipelinePart):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    return _create_default_discovery_part(const_discover_petri_net_alpha_plus)


class DiscoverPetriNetAlphaPlusPlus(PipelinePart):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    return _create_default_discovery_part(const_discover_petri_net_alpha_plus_plus)


class DiscoverPetriNetAlphaPlusPlusNfc(PipelinePart):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    return _create_default_discovery_part(const_discover_petri_net_alpha_plus_plus_nfc)


class DiscoverPetriNetHeuristic(PipelinePart):
  def __init__(self,
               dependency_threshold: float = 0.5,
               positive_observations_threshold: int = 1,
               relative_to_best_threshold: float = 1.0,
               and_threshold: float = 0.1,
               loop_length_two_threshold: float = 0.5):
    super().__init__()
    self.dependency_threshold = dependency_threshold
    self.positive_observations_threshold = positive_observations_threshold
    self.relative_to_best_threshold = relative_to_best_threshold
    self.and_threshold = and_threshold
    self.loop_length_two_threshold = loop_length_two_threshold

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_float_value(config, const_dependency_relation_threshold, self.dependency_threshold)
    append_uint32_value(config, const_positive_observations_threshold, self.positive_observations_threshold)
    append_float_value(config, const_relative_to_best_threshold, self.relative_to_best_threshold)
    append_float_value(config, const_and_threshold, self.and_threshold)
    append_float_value(config, const_loop_length_two_threshold, self.loop_length_two_threshold)

    return GrpcPipelinePartBase(
      defaultPart=create_default_pipeline_part(const_discover_petri_net_heuristic, config))


class DiscoverFuzzyGraph(PipelinePart):
  def __init__(self,
               unary_frequency_threshold: float = 0.0,
               binary_significance_threshold: float = 0.0,
               preserve_threshold: float = 0.0,
               ratio_threshold: float = 0.0,
               utility_rate: float = 0.0,
               edge_cutoff_threshold: float = 0.0,
               node_cutoff_threshold: float = 0.0):
    super().__init__()
    self.unary_frequency_threshold = unary_frequency_threshold
    self.binary_significance_threshold = binary_significance_threshold
    self.preserve_threshold = preserve_threshold
    self.ratio_threshold = ratio_threshold
    self.utility_rate = utility_rate
    self.edge_cutoff_threshold = edge_cutoff_threshold
    self.node_cutoff_threshold = node_cutoff_threshold

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_float_value(config, const_unary_frequency_threshold, self.unary_frequency_threshold)
    append_float_value(config, const_binary_frequency_significance_threshold, self.binary_significance_threshold)
    append_float_value(config, const_preserve_threshold, self.preserve_threshold)
    append_float_value(config, const_ratio_threshold, self.ratio_threshold)
    append_float_value(config, const_utility_rate, self.utility_rate)
    append_float_value(config, const_edge_cutoff_threshold, self.edge_cutoff_threshold)
    append_float_value(config, const_node_cutoff_threshold, self.node_cutoff_threshold)

    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_discover_graph_fuzzy, config))


class SerializePetriNetToPNML(PipelinePart):
  def __init__(self, save_path, use_names_as_ids: bool = False):
    super().__init__()
    self.save_path = save_path
    self.use_names_as_ids = use_names_as_ids

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_string_value(config, const_path, self.save_path)
    append_bool_value(config, const_pnml_use_names_as_ids, self.use_names_as_ids)

    return GrpcPipelinePartBase(
      defaultPart=create_default_pipeline_part(const_serialize_petri_net_to_pnml, config))


class ViewGraphLikeFormalismPart(PipelinePartWithCallback):
  def __init__(self,
               name: str = 'petri_net',
               background_color: str = 'white',
               engine='dot',
               export_path: Optional[str] = None,
               rankdir: str = 'LR'):
    super().__init__()
    self.export_path = export_path
    self.name = name
    self.background_color = background_color
    self.engine = engine
    self.rankdir = rankdir

  def execute_callback(self, values: dict[str, GrpcContextValue]):
    draw_graph(from_grpc_graph(values[const_graph].graph),
               name=self.name,
               background_color=self.background_color,
               engine=self.engine,
               rankdir=self.rankdir,
               export_path=self.export_path)


class ViewPetriNet(ViewGraphLikeFormalismPart):
  def __init__(self,
               show_places_names: bool = False,
               name: str = 'petri_net',
               background_color: str = 'white',
               engine='dot',
               export_path: Optional[str] = None,
               rankdir: str = 'LR',
               annotation: dict[int, str] = None):
    super().__init__(name, background_color, engine, export_path, rankdir)
    self.show_places_names = show_places_names
    self.annotation = annotation

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    part = create_simple_get_context_value_part(self.uuid, self.__class__.__name__, const_petri_net)
    return GrpcPipelinePartBase(simpleContextRequestPart=part)

  def execute_callback(self, values: dict[str, GrpcContextValue]):
    draw_petri_net(from_grpc_petri_net(values[const_petri_net].petriNet),
                   show_places_names=self.show_places_names,
                   name=self.name,
                   background_color=self.background_color,
                   engine=self.engine,
                   rankdir=self.rankdir,
                   export_path=self.export_path,
                   annotation=self.annotation)


class ViewDirectlyFollowsGraphBase(ViewGraphLikeFormalismPart):
  def __init__(self,
               dfg_discovery_part_name: str,
               name: str = 'dfg_graph',
               background_color: str = 'white',
               engine='dot',
               export_path: Optional[str] = None,
               rankdir: str = 'LR'):
    super().__init__(name, background_color, engine, export_path, rankdir)
    self.dfg_discovery_part_name = dfg_discovery_part_name

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    part = create_complex_get_context_part(self.uuid,
                                           self.__class__.__name__,
                                           [const_graph],
                                           self.dfg_discovery_part_name,
                                           GrpcPipelinePartConfiguration())

    return GrpcPipelinePartBase(complexContextRequestPart=part)


class ViewDirectlyFollowsGraph(ViewDirectlyFollowsGraphBase):
  def __init__(self,
               name: str = 'dfg_graph',
               background_color: str = 'white',
               engine='dot',
               export_path: Optional[str] = None,
               rankdir: str = 'LR'):
    super().__init__(const_discover_directly_follows_graph, name, background_color, engine, export_path, rankdir)


class ViewDirectlyFollowsGraphStream(ViewDirectlyFollowsGraphBase):
  def __init__(self,
               name: str = 'dfg_graph',
               background_color: str = 'white',
               engine='dot',
               export_path: Optional[str] = None,
               rankdir: str = 'LR'):
    super().__init__(const_discover_directly_follows_graph_stream, name, background_color, engine, export_path, rankdir)


class DiscoverDirectlyFollowsGraph(PipelinePart):
  def __init__(self, thread_attribute: Optional[str] = None):
    super().__init__()
    self.thread_attribute = thread_attribute

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    if self.thread_attribute is not None:
      append_string_value(config, const_thread_attribute, self.thread_attribute)

    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_discover_directly_follows_graph, config))


class DiscoverDirectlyFollowsGraphByAttribute(PipelinePart):
  def __init__(self, attribute: str):
    super().__init__()
    self.attribute = attribute

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_string_value(config, const_attribute, self.attribute)
    part = create_default_pipeline_part(const_discover_directly_follows_graph_by_attribute, config)

    return GrpcPipelinePartBase(defaultPart=part)


class ViewGraph(ViewGraphLikeFormalismPart):
  def __init__(self,
               name: str = 'dfg_graph',
               background_color: str = 'white',
               engine='dot',
               export_path: Optional[str] = None,
               rankdir: str = 'LR'):
    super().__init__(name, background_color, engine, export_path, rankdir)

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    part = create_simple_get_context_value_part(self.uuid, self.__class__.__name__, const_graph)
    return GrpcPipelinePartBase(simpleContextRequestPart=part)


class EnsureInitialMarking(PipelinePart):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_ensure_initial_marking))


class DiscoverDirectlyFollowsGraphStream(PipelinePart):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    return _create_default_discovery_part(const_discover_directly_follows_graph_stream)

class DiscoverLCSGraph(PipelinePart):
  def __init__(self, root_sequence_kind: RootSequenceKind = RootSequenceKind.FindBest):
    super().__init__()
    self.root_sequence_kind = root_sequence_kind

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_root_sequence_kind(config, const_root_sequence_kind, self.root_sequence_kind)

    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_discover_lcs_graph, config))
