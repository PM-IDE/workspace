from .entry_points.default_pipeline import *
from .models.pipelines_and_context_pb2 import *
from ..legacy.discovery.graph import draw_graph
from ..legacy.discovery.petri_net import draw_petri_net
from ..legacy.util import RandomUniqueColorsProvider


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
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return _create_default_discovery_part(const_discover_directly_follows_graph)


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

class DiscoverLogTimelineDiagram(PipelinePartWithCallback):
  def __init__(self,
               thread_attribute: str,
               time_attribute: Optional[str],
               title: Optional[str] = None,
               save_path: str = None,
               plot_legend: bool = False,
               height_scale: float = 1,
               width_scale: float = 1,
               distance_scale: float = 1,
               rect_width_scale: int = 1):
    super().__init__()
    self.thread_attribute = thread_attribute
    self.time_attribute = time_attribute
    self.title = title
    self.save_path = save_path
    self.plot_legend = plot_legend
    self.height_scale = height_scale
    self.width_scale = width_scale
    self.distance_scale = distance_scale
    self.rect_width_scale = rect_width_scale

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_string_value(config, const_thread_attribute, self.thread_attribute)

    if self.time_attribute is not None:
      append_string_value(config, const_time_attribute, self.time_attribute)

    part = create_complex_get_context_part(self.uuid,
                                           self.__class__.__name__,
                                           [const_log_timeline_diagram],
                                           const_discover_log_timeline_diagram,
                                           config)

    return GrpcPipelinePartBase(complexContextRequestPart=part)

  def execute_callback(self, values: dict[str, GrpcContextValue]):
    diagram = values[const_log_timeline_diagram].logTimelineDiagram

    black = (0, 0, 0)
    white = (255, 255, 255)
    provider = RandomUniqueColorsProvider(used_colors={black, white})
    colors = dict()
    background_key = 'Background'
    separator_key = 'Separator'
    rect_width = self.rect_width_scale
    colors[background_key] = 0
    colors[separator_key] = 1
    mappings = [
      ProxyColorMapping(background_key, Color(white[0], white[1], white[2])),
      ProxyColorMapping(separator_key, Color(black[0], black[1], black[2]))
    ]

    max_stamp = 0
    max_events = 0
    for trace_diagram in diagram.traces:
      for thread in trace_diagram.threads:
        max_stamp = max(max_stamp, thread.events[-1].stamp)
        max_events = max(max_events, len(thread.events))

    colors_log = []
    for trace_diagram in diagram.traces:
      for thread in trace_diagram.threads:
        colors_trace = []
        last_x = 0
        for event in thread.events:
          if event.name not in colors:
            c = provider.next()
            mappings.append(ProxyColorMapping(event.name, Color(c[0], c[1], c[2])))
            colors[event.name] = len(colors)

          rect_x = event.stamp * self.distance_scale
          if last_x != rect_x:
            colors_trace.append(ProxyColorRectangle(
              colors[background_key],
              last_x,
              rect_x - last_x
            ))

          colors_trace.append(ProxyColorRectangle(
            colors[event.name],
            rect_x,
            rect_width,
          ))

          last_x = rect_x + rect_width

        colors_log.append(ProxyColorsTrace(colors_trace, False))

      colors_log.append(ProxyColorsTrace([ProxyColorRectangle(
        colors[separator_key],
        0,
        max_stamp * self.distance_scale + max_events * self.rect_width_scale
      )], False))

    draw_colors_event_log_canvas(ProxyColorsEventLog(mappings, colors_log),
                                 title=self.title,
                                 save_path=self.save_path,
                                 plot_legend=self.plot_legend,
                                 height_scale=self.height_scale,
                                 width_scale=self.width_scale)
