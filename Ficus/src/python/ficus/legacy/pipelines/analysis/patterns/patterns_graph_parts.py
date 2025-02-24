import os
from typing import Callable

import graphviz

from .patterns_parts import GraphAttributesSetter, InternalGraphDrawingPart
from .type_aliases import NodeNameCreator
from ...contexts.accessors import activities, graph, log, event_class_tree
from ...contexts.part_results import PipelinePartResult
from ....analysis.patterns.event_log_patterns import build_class_tree
from ....analysis.patterns.patterns_graphs import default_graph_attr_setter, default_node_name_creator, build_graph, \
  _do_draw_graph
from ....analysis.patterns.util import split_activities_by_levels_dict, split_activities_by_level
from ....pipelines.pipelines import InternalPipelinePart, WithInputCopy, Pipeline


class InternalBuildGraphPart(InternalPipelinePart):
  def __init__(self,
               graph_name: str,
               add_root_node: bool = True,
               graph_attributes_setter: GraphAttributesSetter = default_graph_attr_setter,
               node_name_creator: NodeNameCreator = default_node_name_creator):
    self.graph_name = graph_name
    self.add_root_node = add_root_node
    self.graph_attributes_setter = graph_attributes_setter
    self.node_name_creator = node_name_creator

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    raise NotImplementedError()


class BuildActivityGraph(InternalBuildGraphPart):
  def __init__(self,
               activity_level: int,
               graph_name: str,
               add_root_node: bool = True,
               graph_attributes_setter: GraphAttributesSetter = default_graph_attr_setter,
               node_name_creator: NodeNameCreator = default_node_name_creator):
    super().__init__(graph_name, add_root_node, graph_attributes_setter, node_name_creator)
    self.activity_level = activity_level

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    current_activities = activities(current_input)
    activities_for_level = split_activities_by_levels_dict(current_activities)[self.activity_level]
    built_graph = build_graph(nodes=activities_for_level,
                              graph_name=self.graph_name,
                              add_root_node=self.add_root_node,
                              set_attributes_to_func=self.graph_attributes_setter,
                              node_name_creator=self.node_name_creator)

    return current_input.with_graph(built_graph)


def default_activities_level_save_path_mutator(base_path: str, activity_level: int) -> str:
  names = os.path.splitext(base_path)
  name_without_ext = names[0]
  file_ext = names[1]
  return f'{name_without_ext}_{activity_level}{file_ext}'


class DrawActivitiesGraphByEventClassLevels(InternalGraphDrawingPart):
  def __init__(self,
               save_path: str = None,
               use_hashes_as_name: bool = True,
               add_root_node: bool = True,
               graph_attributes_setter: Callable[[graphviz.Digraph], None] = default_graph_attr_setter,
               node_name_creator: NodeNameCreator = default_node_name_creator,
               save_path_mutator: Callable[[str, int], str] = default_activities_level_save_path_mutator):
    super().__init__(save_path=save_path,
                     add_root_node=add_root_node,
                     graph_attributes_setter=graph_attributes_setter)
    self.use_hashes_as_name = use_hashes_as_name
    self.node_name_creator = node_name_creator
    self.save_path_mutator = save_path_mutator

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    activities_by_levels = split_activities_by_level(activities(current_input))
    for level, _ in activities_by_levels:
      current_save_path = self.save_path_mutator(self.save_path, level) if self.save_path is not None else None
      WithInputCopy(
        Pipeline(
          BuildActivityGraph(activity_level=level,
                             graph_name='Activities Graph',
                             add_root_node=self.add_root_node,
                             node_name_creator=self.node_name_creator,
                             graph_attributes_setter=self.graph_attributes_setter),
          DrawGraph(current_save_path)
        )
      )(current_input)

    return current_input


class DrawGraph(InternalPipelinePart):
  def __init__(self, save_path: str = None):
    self.save_path = save_path

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    _do_draw_graph(graph(current_input), self.save_path)
    return current_input


class SerializeGraph(InternalPipelinePart):
  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    return current_input.with_serialized_graph(serialize_graphviz_graph(graph(current_input)))


def serialize_graphviz_graph(graph: graphviz.Digraph) -> str:
  lines = str(graph).split('\n')
  return '\n'.join(sorted(lines[1:(len(lines) - 1)]))


class BuildEventClassTree(InternalPipelinePart):
  def __init__(self, class_extractors: list[Callable[[str], str]]):
    self.class_extractors = class_extractors

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    tree = build_class_tree(log(current_input), self.class_extractors)
    return current_input.with_event_class_tree(tree)


class DrawEventClassTree(InternalGraphDrawingPart):
  def __init__(self,
               add_root_node: bool = True,
               save_path: str = None,
               graph_attributes_setter: Callable[[graphviz.Digraph], None] = default_graph_attr_setter):
    super().__init__(save_path=save_path,
                     add_root_node=add_root_node,
                     graph_attributes_setter=graph_attributes_setter)

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    WithInputCopy(
      Pipeline(
        BuildEventClassGraph(graph_name='Event Class Graph',
                             add_root_node=self.add_root_node,
                             graph_attributes_setter=self.graph_attributes_setter),
        DrawGraph(save_path=self.save_path),
      )
    )(current_input)

    return current_input


class BuildEventClassGraph(InternalBuildGraphPart):
  def __init__(self,
               graph_name: str,
               add_root_node: bool = True,
               graph_attributes_setter: GraphAttributesSetter = default_graph_attr_setter,
               node_name_creator: NodeNameCreator = default_node_name_creator):
    super().__init__(graph_name, add_root_node, graph_attributes_setter, node_name_creator)

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    graph = build_graph(event_class_tree(current_input),
                        graph_name=self.graph_name,
                        add_root_node=self.add_root_node,
                        node_name_creator=self.node_name_creator,
                        set_attributes_to_func=self.graph_attributes_setter)

    return current_input.with_graph(graph)
