import dataclasses
from typing import Callable, Optional

import graphviz
from IPython.core.display import Image
from IPython.core.display_functions import display
from attr import dataclass
from matplotlib import pyplot as plt
from matplotlib.collections import PatchCollection

from .constants import undefined_activity, activity_name_sep
from ..common.common_models import GraphNode, SubArrayInEventLog
from ..event_log_analysis import TraceDiversityLikeDiagramContext, _draw_traces_diversity_like_diagram_internal
from ...analysis.patterns.patterns_models import ActivityInTraceInfo, ActivityNode, EventClassNode
from ...log.event_log import MyEventLog
from ...util import to_hex, calculate_poly_hash_for_collection, SingleColorProvider, \
    random_unique_color_provider_instance


def draw_full_activity_diagram(log: MyEventLog,
                               traces_activities: list[list[ActivityInTraceInfo]],
                               cached_colors: dict[str, str],
                               title: str = None,
                               plot_legend: bool = True,
                               save_path: str = None,
                               height_scale: int = 1,
                               width_scale: int = 1):
    _do_draw_activity_diagram(log, traces_activities, cached_colors,
                              title, False, plot_legend, save_path, height_scale, width_scale)


def draw_short_activity_diagram(log: MyEventLog,
                                traces_activities: list[list[ActivityInTraceInfo]],
                                cached_colors: dict[str, str],
                                title: str = None,
                                plot_legend: bool = True,
                                save_path: str = None,
                                height_scale: int = 1,
                                width_scale: int = 1):
    _do_draw_activity_diagram(log, traces_activities, cached_colors,
                              title, True, plot_legend, save_path, height_scale, width_scale)


def _do_draw_activity_diagram(log: MyEventLog,
                              traces_activities: list[list[ActivityInTraceInfo]],
                              cached_colors: dict[str, str],
                              title: str,
                              short_diagram: bool,
                              plot_legend: bool,
                              save_path: str,
                              height_scale: int,
                              width_scale: int):
    def draw_func(ctx: TraceDiversityLikeDiagramContext):
        dtos = []
        for trace_activities in traces_activities:
            trace_dtos = []
            for activity in trace_activities:
                events = activity.node.set_of_events
                activity_hash = calculate_poly_hash_for_collection(list(sorted(events)))

                trace_dtos.append(SubTraceDrawingDto(
                    activity.start_pos,
                    activity.length,
                    activity_hash,
                    activity.node.name
                ))

            dtos.append(trace_dtos)

        _sub_traces_draw_func(ctx, log, cached_colors, dtos, short_diagram)

    _draw_traces_diversity_like_diagram_internal(log, draw_func, title, save_path, plot_legend, height_scale, width_scale)


@dataclass
class SubTraceDrawingDto:
    start_pos: int
    length: int
    hash: int
    name: str


def _sub_traces_draw_func(ctx: TraceDiversityLikeDiagramContext,
                          log: MyEventLog,
                          cached_colors: dict[str, str],
                          activities: list[list[SubTraceDrawingDto]],
                          draw_short_diagram: bool):
    real_colors_provider = random_unique_color_provider_instance

    def colors_provider(name: str):
        if name in cached_colors:
            return cached_colors[name]

        if name == undefined_activity:
            return to_hex((0, 0, 0))

        generated_color = real_colors_provider.next()
        cached_colors[name] = generated_color
        return generated_color

    current_y = 0
    activities_colors = dict()

    for trace_activities, real_trace in zip(activities, log):
        if len(trace_activities) == 0:
            width = ctx.rect_width
            if not draw_short_diagram:
                width *= len(real_trace)

            rect = plt.Rectangle((0, current_y), width, ctx.rect_height, fc=colors_provider(undefined_activity))
            ctx.names_to_rects[undefined_activity] = rect
            ctx.drawer.add_patch(rect)
        else:
            current_x = 0
            last_drew_index = 0
            patches = []

            for index, dto in enumerate(trace_activities):
                if last_drew_index < dto.start_pos:
                    width = ctx.rect_width
                    if not draw_short_diagram:
                        width *= (dto.start_pos - last_drew_index)

                    rect = plt.Rectangle((current_x, current_y), width, ctx.rect_height,
                                         fc=colors_provider(undefined_activity))
                    ctx.names_to_rects[undefined_activity] = rect
                    patches.append(rect)
                    current_x += width


                activity_name = dto.name
                if dto.hash not in activities_colors:
                    activities_colors[dto.hash] = colors_provider(activity_name)

                activity_x_width = ctx.rect_width
                if not draw_short_diagram:
                    activity_x_width *= dto.length

                color = activities_colors[dto.hash]
                rect = plt.Rectangle((current_x, current_y), activity_x_width, ctx.rect_height, fc=color)
                ctx.names_to_rects[activity_name] = rect
                patches.append(rect)
                current_x += activity_x_width
                last_drew_index = dto.start_pos + dto.length

            if last_drew_index < len(real_trace):
                width = ctx.rect_width
                if not draw_short_diagram:
                    width *= (len(real_trace) - last_drew_index)

                rect = plt.Rectangle((current_x, current_y), width, ctx.rect_height,
                                     fc=colors_provider(undefined_activity))
                patches.append(rect)

            ctx.drawer.add_collection(PatchCollection(patches, match_original=True))

        current_y += ctx.rect_height + ctx.y_delta_between_traces


def default_graph_attr_setter(_: graphviz.Digraph):
    pass


def draw_activity_graph(activities: list[ActivityNode],
                        save_path: str = None,
                        use_hashes_as_name: bool = True,
                        add_root_node: bool = True,
                        set_attributes_to_func: Callable[[graphviz.Digraph], None] = default_graph_attr_setter):
    def create_name(node: GraphNode):
        def preprocess_name(activity_name: str):
            names = activity_name.split(activity_name_sep)
            return '\n'.join(names)

        return str(hash(node)) if use_hashes_as_name else preprocess_name(node.name)

    _draw_graph(activities,
                'Activities Graph',
                save_path=save_path,
                node_name_creator=create_name,
                add_root_node=add_root_node,
                set_attributes_to_func=set_attributes_to_func)


def default_node_name_creator(node: GraphNode):
    return node.name


def _draw_graph(nodes: list[GraphNode],
                graph_name: str,
                save_path: str = None,
                add_root_node: bool = True,
                node_name_creator: Callable[[GraphNode], str] = default_node_name_creator,
                set_attributes_to_func: Callable[[graphviz.Digraph], None] = default_graph_attr_setter):
    graph = build_graph(nodes, graph_name, add_root_node, node_name_creator, set_attributes_to_func)
    _do_draw_graph(graph, save_path)


def _do_draw_graph(graph: graphviz.Digraph, save_path: Optional[str] = None):
    if save_path is not None:
        graph.render(filename=save_path, format='png')
    else:
        graph.format = 'png'
        image = Image(graph.render())
        display(image)


def build_graph(nodes: list[GraphNode],
                graph_name: str,
                add_root_node: bool = True,
                node_name_creator: Callable[[GraphNode], str] = default_node_name_creator,
                set_attributes_to_func: Callable[[graphviz.Digraph], None] = default_graph_attr_setter):
    graph = graphviz.Digraph(graph_name)
    added_names = set()

    def add_node(current_graph: graphviz.Digraph, parent_node: str, node: GraphNode):
        name = node_name_creator(node)
        if name in added_names:
            if parent_node is not None:
                current_graph.edge(name, parent_node)

            return

        added_names.add(name)

        if parent_node is not None:
            current_graph.edge(name, parent_node)

        current_graph.node(name)
        for child_activity in node.child_nodes:
            add_node(current_graph, name, child_activity)

    root_name = None if not add_root_node else 'root'
    for top_level_node in nodes:
        add_node(graph, root_name, top_level_node)

    set_attributes_to_func(graph)
    return graph


def draw_activity_placement_diagram(log: MyEventLog,
                                    activity_node: ActivityNode,
                                    traces_activities: list[list[ActivityInTraceInfo]],
                                    use_different_colors: bool = False,
                                    plot_legend: bool = False,
                                    title: str = None,
                                    height_scale: int = 1,
                                    save_path: str = None):
    def draw_func(ctx: TraceDiversityLikeDiagramContext):
        color_provider = random_unique_color_provider_instance if use_different_colors else SingleColorProvider()
        undefined_color = to_hex((0, 0, 0))
        current_y = 0
        undefined_activity_name = 'UndefinedActivity'

        for trace_activities, trace in zip(traces_activities, log):
            last_draw_index = 0
            current_x = 0
            patches = []

            for activity in trace_activities:
                if activity.node.name == activity_node.name:
                    if last_draw_index < activity.start_pos:
                        width = (activity.start_pos - last_draw_index) * ctx.rect_width
                        rect = plt.Rectangle((current_x, current_y), width, ctx.rect_height, fc=undefined_color)
                        ctx.names_to_rects[undefined_activity_name] = rect
                        patches.append(rect)
                        last_draw_index = activity.start_pos
                        current_x += width

                    width = activity.length * ctx.rect_width
                    rect = plt.Rectangle((current_x, current_y), width, ctx.rect_height, fc=color_provider.next())
                    ctx.names_to_rects[activity.node.name] = rect
                    patches.append(rect)
                    last_draw_index += activity.length
                    current_x += width

            if last_draw_index < len(trace):
                width = len(trace) - last_draw_index
                rect = plt.Rectangle((current_x, current_y), width, ctx.rect_height, fc=undefined_color)
                ctx.names_to_rects[undefined_activity_name] = rect
                patches.append(rect)

            ctx.drawer.add_collection(PatchCollection(patches, match_original=True))
            current_y += ctx.rect_height
            color_provider.reset()

    _draw_traces_diversity_like_diagram_internal(log,
                                                 draw_func,
                                                 plot_legend=plot_legend,
                                                 height_scale=height_scale,
                                                 title=title,
                                                 save_path=save_path)


def draw_event_class_tree(nodes: list[EventClassNode],
                          save_path: str = None,
                          add_root_node: bool = True,
                          set_attributes_to_func: Callable[[graphviz.Digraph], None] = default_graph_attr_setter):
    _draw_graph(nodes,
                'Event Classes Graph',
                save_path=save_path,
                add_root_node=add_root_node,
                set_attributes_to_func=set_attributes_to_func)


def draw_patterns(log: MyEventLog,
                  traces_patterns: list[list[SubArrayInEventLog]],
                  cached_colors: dict[str, str],
                  title: str = None,
                  plot_legend: bool = True,
                  save_path: str = None,
                  height_scale: int = 1,
                  width_scale: int = 1,
                  short_diagram: bool = False):
    def draw_func(ctx: TraceDiversityLikeDiagramContext):
        dtos = []
        for trace, trace_patterns in zip(log, traces_patterns):
            trace_dtos = []
            for pattern in trace_patterns:
                pattern_seq = trace[pattern.first_pos:(pattern.first_pos + pattern.length)]
                pattern_hash = calculate_poly_hash_for_collection(pattern_seq)
                trace_dtos.append(SubTraceDrawingDto(
                    pattern.first_pos,
                    pattern.length,
                    pattern_hash,
                    str(pattern_hash)
                ))

            dtos.append(trace_dtos)

        _sub_traces_draw_func(ctx, log, cached_colors, dtos, short_diagram)

    _draw_traces_diversity_like_diagram_internal(log, draw_func, title, save_path, plot_legend, height_scale,
                                                 width_scale)
